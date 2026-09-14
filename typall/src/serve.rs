//! 开发服务器：静态文件 + 文件监听 + Live Reload。
//!
//! Live Reload 经 SSE（`/__events`）推送构建状态：版本变化 → 软刷新
//! （仅替换 `main#content`，滚动位置保留；主题/模板结构变化时回退整页刷新），
//! 构建失败 → 页面底部错误浮层。脚本仅在 `serve` 时临时注入到 HTML
//! 响应中，**不会写入磁盘**，因此 `build` 产物仍保持零 JS。

use std::convert::Infallible;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, Instant};

use axum::extract::{Path as AxumPath, State};
use axum::http::{header, HeaderMap};
use axum::response::sse::{Event, KeepAlive, Sse};
use axum::response::Response;
use axum::routing::get;
use axum::Router;
use notify::event::{ModifyKind, RenameMode};
use notify::{EventKind, RecursiveMode, Watcher};
use tokio::sync::watch;
use tokio_stream::wrappers::WatchStream;
use tokio_stream::{Stream, StreamExt};

use crate::build;
use crate::config::Config;

/// 一次构建尝试的结果（SSE `build` 事件推送给浏览器）。
#[derive(Clone, serde::Serialize)]
struct BuildStatus {
    version: u64,
    /// 编译错误全文；`None` = 构建成功。
    error: Option<String>,
}

/// 注入到 HTML 的 Live Reload 脚本（SSE 版：监听 `/__events`）。
///
/// - `build` 事件携带 `{"version": N, "error": string|null}`；watch 通道在
///   订阅时立即产出当前值，后打开的页面也能立刻进入正确状态。
/// - 构建失败 → 页面底部固定错误浮层（**追加不覆盖**，可关闭）；
/// - 构建成功且版本变化 → **软刷新**：fetch 当前页新 HTML，仅替换
///   `main#content` 与 `<title>`，滚动位置不丢。顶栏结构不一致（主题/模板
///   变更）或 fetch 失败时回退整页 `location.reload()`（浮层随之消失）。
const LIVE_RELOAD_SCRIPT: &str = r#"<script>
(function(){
  var v=null;
  var es=new EventSource('/__events');
  function fullReload(){location.reload()}
  // 软刷新：fetch 当前页新 HTML，仅替换 main#content 与 <title>，滚动位置不丢。
  // fetch 拿不到可用页面（构建写盘窗口/网络抖动）时先重试一次，仍失败才整页刷新。
  function softRefresh(ver,retried){
    fetch(location.href,{cache:'no-store'}).then(function(r){
      if(!r.ok){throw 0}
      return r.text()
    }).then(function(html){
      var doc=new DOMParser().parseFromString(html,'text/html');
      var curMain=document.querySelector('main#content');
      var freshMain=doc.querySelector('main#content');
      var curHead=document.querySelector('header.site-header');
      var freshHead=doc.querySelector('header.site-header');
      // 无内容容器，或顶栏结构变化（主题/模板改了，软刷新覆盖不到 header）→ 整页
      if(!curMain||!freshMain||
         (freshHead?freshHead.outerHTML:'')!==(curHead?curHead.outerHTML:'')){
        fullReload();return
      }
      curMain.innerHTML=freshMain.innerHTML;
      var t=document.querySelector('title'),nt=doc.querySelector('title');
      if(t&&nt){t.textContent=nt.textContent}
      v=ver;
    }).catch(function(){
      if(retried){fullReload();return}
      setTimeout(function(){softRefresh(ver,true)},400);
    });
  }
  es.addEventListener('build',function(e){
    var d;try{d=JSON.parse(e.data)}catch(err){return}
    if(d.error){
      var box=document.getElementById('__typall-build-errors');
      if(!box){
        box=document.createElement('div');box.id='__typall-build-errors';
        box.setAttribute('style','position:fixed;left:0;right:0;bottom:0;z-index:99999;max-height:45vh;overflow:auto;background:#2b1516;color:#ffb4ae;font:12px/1.6 Consolas,monospace;padding:10px 14px;border-top:2px solid #e5534b;white-space:pre-wrap');
        var bar=document.createElement('div');bar.setAttribute('style','text-align:right;margin-bottom:6px');
        var close=document.createElement('button');close.textContent='关闭 ✕';
        close.setAttribute('style','background:none;border:1px solid #7a4040;color:#ffb4ae;cursor:pointer;border-radius:4px;padding:2px 8px;font-size:12px');
        close.onclick=function(){box.remove()};
        bar.appendChild(close);box.appendChild(bar);
        document.body.appendChild(box);
      }
      var item=document.createElement('div');
      item.textContent='⚠ 构建失败\n'+d.error+'\n';
      box.appendChild(item);
    }else if(v===null){
      v=d.version;
    }else if(d.version!==v){
      softRefresh(d.version);
    }
  });
})();
</script>"#;

/// 「复制到公众号」浮动按钮 + 剪贴板脚本（仅 serve 时注入，build 产物零 JS）。
///
/// 复制时实时抓取 `<article>` DOM 的克隆副本处理：
///   1. 把公式/插图 `<svg>` 里的 `<defs><symbol id>` + `<use xlink:href>` 结构
///      **use/symbol 展开**成纯 `<path>`（删光 id/defs/xlink）——绕开公众号保存层
///      删 id/defs 导致公式空白的问题，同时保留矢量。
///   2. 普通 `<img src="相对路径">` 转绝对 URL（依赖 site.url 正确解析页面 URL）。
///   3. 双格式写入剪贴板（text/html + text/plain），公众号编辑器吃 HTML 富文本。
///      操作的是 cloneNode 副本，不污染页面原 DOM。
const COPY_SCRIPT: &str = r#"<style>
.copy-float{position:fixed;right:16px;top:40%;z-index:9999;display:flex;flex-direction:column;gap:8px}
.copy-float button{font:14px/1.4 system-ui,sans-serif;padding:8px 12px;border:1px solid #ddd;border-radius:8px;background:#fff;color:#333;cursor:pointer;box-shadow:0 2px 8px rgba(0,0,0,.08);transition:.15s}
.copy-float button:hover{border-color:#07c160;color:#07c160}
.copy-float button.done{border-color:#07c160;background:#07c160;color:#fff}
@media (max-width:640px){.copy-float{right:8px;top:auto;bottom:12px;flex-direction:row}}
</style>
<script>
(function(){
  if(!document.querySelector('article')){return}
  function esc(s){return String(s).replace(/&/g,'&amp;').replace(/</g,'&lt;').replace(/>/g,'&gt;')}
  // use/symbol 展开：返回处理后的 svg，页面 DOM 不变
  function expandSvg(orig){
    var svg=orig.cloneNode(true), ns='http://www.w3.org/2000/svg';
    var symbols={};
    Array.prototype.forEach.call(svg.querySelectorAll('symbol'),function(sym){
      symbols[sym.getAttribute('id')]=sym
    });
    // 删掉所有 defs（微信保存层会删 defs/id）
    Array.prototype.forEach.call(svg.querySelectorAll('defs'),function(d){d.remove()});
    // 继承 use 上的展示属性（typst 的 symbol 内 path 不带 fill，靠 use 上色）
    var PRES=['fill','fill-rule','stroke','stroke-width','stroke-linecap','stroke-linejoin','stroke-miterlimit','opacity','color','fill-opacity','stroke-opacity'];
    function inherit(use,g){
      PRES.forEach(function(p){
        if(use.hasAttribute(p)&&!g.hasAttribute(p)){g.setAttribute(p,use.getAttribute(p))}
      })
    }
    Array.prototype.forEach.call(svg.querySelectorAll('use'),function(use){
      var href=use.getAttribute('xlink:href')||use.getAttribute('href');
      if(!href||href.charAt(0)!=='#'){return}
      var sym=symbols[href.slice(1)];
      if(!sym){return}
      var g=document.createElementNS(ns,'g');
      Array.prototype.forEach.call(sym.childNodes,function(c){
        if(c.nodeType===1){g.appendChild(c.cloneNode(true))}
      });
      // x/y 平移 → translate（外层若有 <g transform> 包裹，replaceChild 后该外层
      // transform 依旧生效，因此这里只处理 use 自身的 x/y，避免 transform 叠加两次）
      var x=parseFloat(use.getAttribute('x'))||0, y=parseFloat(use.getAttribute('y'))||0;
      if(x||y){g.setAttribute('transform','translate('+x+','+y+')')}
      inherit(use,g);
      use.parentNode.replaceChild(g,use)
    });
    // svg 自身若有 data-equation 包裹的编号 span（eq-num），由外层容器统一处理
    return svg
  }
  // 图片相对路径转绝对 URL
  function absolutize(container){
    Array.prototype.forEach.call(container.querySelectorAll('img[src]'),function(img){
      var src=img.getAttribute('src');
      if(src&&!/^(https?:|data:|blob:)/i.test(src)){
        img.setAttribute('src',new URL(src,location.href).href)
      }
    })
  }
  // —— 公众号只认内联 style：以下把预览页外部 css 的正文排版写成元素内联 style，
  //    色值固定为浅色主题（公众号无暗色），字号统一 px，避免 rem/em 依赖根字号失效。
  //    色值与 src/theme.rs 的「墨理」主题对齐（媒体内 CSS-inliner 仅取亮色令牌）。
  var C_FG='#20222a', C_MUT='#686a72', C_ACC='#0b6fc0', C_BRD='#e6e2d6',
      C_CODE='#f3f1ec', C_CRD='#ffffff', C_QUOTE='#3c5a49', C_QUOTE_SOFT='#eef2ec';
  var FONT='font-family:-apple-system,"PingFang SC","Microsoft YaHei","Helvetica Neue",sans-serif;';
  var MONO='font-family:"SFMono-Regular",Consolas,"DejaVu Sans Mono",monospace;';
  function styleOf(el,css){ // 合并进已有 style，不覆盖 svg/现有内联
    var pre=el.getAttribute('style'); el.setAttribute('style',(pre?pre.replace(/;?$/,';'):'')+css)
  }
  function tagStyle(el){
    var t=el.tagName.toLowerCase(), css='';
    if(t==='h1')css='font-size:28px;font-weight:bold;line-height:1.3;margin:34px 0 16px;'+FONT+'color:'+C_FG+';';
    else if(t==='h2')css='font-size:22px;font-weight:bold;line-height:1.3;margin:34px 0 14px;padding-bottom:6px;border-bottom:1px solid '+C_BRD+';'+FONT+'color:'+C_FG+';';
    else if(t==='h3'||t==='h4')css='font-size:'+(t==='h3'?'19':'17')+'px;font-weight:bold;line-height:1.3;margin:26px 0 10px;'+FONT+'color:'+C_FG+';';
    else if(t==='p')css='font-size:16px;line-height:1.8;margin:14px 0;'+FONT+'color:'+C_FG+';';
    else if(t==='blockquote')css='font-size:16px;line-height:1.8;margin:18px 0;padding:8px 18px;border-left:3px solid '+C_QUOTE+';background:'+C_QUOTE_SOFT+';color:'+C_MUT+';'+FONT+';';
    else if(t==='pre')css='background:'+C_CODE+';padding:14px 16px;border-radius:8px;overflow-x:auto;font-size:14px;line-height:1.6;color:'+C_FG+';'+MONO+';';
    else if(t==='ul'||t==='ol')css='margin:14px 0;padding-left:26px;line-height:1.8;'+FONT+'color:'+C_FG+';';
    else if(t==='li')css='margin:4px 0;';
    else if(t==='table')css='border-collapse:collapse;width:100%;margin:16px 0;';
    else if(t==='th'||t==='td')css='border:1px solid '+C_BRD+';padding:8px 12px;text-align:left;font-size:15px;line-height:1.6;'+FONT+';';
    else if(t==='hr')css='border:none;border-top:1px solid '+C_BRD+';margin:26px 0;';
    else if(t==='a')css='color:'+C_ACC+';';
    else if(t==='img')css='max-width:100%;height:auto;border-radius:6px;';
    if(css)styleOf(el,css)
  }
  // 处理整篇 article → 返回清理后适合公众号粘贴的 HTML 字符串
  function buildWechatHtml(){
    var art=document.querySelector('article').cloneNode(true);
    // 站点导航/装饰元素不入剪贴板（h1+正文 + 底部说明保留）
    Array.prototype.forEach.call(art.querySelectorAll('nav.toc,.prev,.next,.meta,.toc,link,script,style'),function(e){e.remove()});
    // ① use/symbol 展开（SVG 保矢量；依赖 id/defs 的引用实体化）
    Array.prototype.forEach.call(art.querySelectorAll('svg'),function(s){
      var ex=expandSvg(s); s.parentNode.replaceChild(ex,s)
    });
    // ② 图片相对路径转绝对
    absolutize(art);
    // ③ 正文层级样式内联（浅色写死），article 也套基准字体，防微信剥容器
    styleOf(art,FONT+'font-size:16px;line-height:1.8;color:'+C_FG+';');
    ['h1','h2','h3','h4','p','blockquote','pre','ul','ol','li','table','th','td','hr','a','img'].forEach(function(t){
      Array.prototype.forEach.call(art.querySelectorAll(t),tagStyle)
    });
    // 行内 code 与 pre 内 code 的衬底/字体（公众号保留 font-family）
    Array.prototype.forEach.call(art.querySelectorAll('code'),function(el){
      if(el.closest('pre'))styleOf(el,MONO+'font-size:0.9em;')
      else styleOf(el,'background:'+C_CODE+';padding:1px 6px;border-radius:4px;'+MONO+';font-size:0.9em;')
    });
    // ④ block 公式：flex/绝对定位(预览用)公众号不支持 → 改「公式居中 + 编号单独右对齐行」。
    //    容器 text-align:center 使行内 svg 居中；eq-num 变 display:block 落到公式下一行靠右。
    Array.prototype.forEach.call(art.querySelectorAll('div[data-equation="block"]'),function(d){
      styleOf(d,'text-align:center;margin:18px 0;');
      Array.prototype.forEach.call(d.querySelectorAll(':scope > svg'),function(sv){
        var st=sv.getAttribute('style')||'';
        sv.setAttribute('style',st.replace(/display\s*:\s*block\s*;?/i,'')+';vertical-align:middle;');
      });
      Array.prototype.forEach.call(d.querySelectorAll(':scope > .eq-num'),function(sp){
        styleOf(sp,'display:block;text-align:right;margin:5px 0 0;font-size:15px;color:'+C_MUT+';'+FONT)
      })
    });
    return art.outerHTML
  }
  function writeClip(feedback){
    try{
      var html=buildWechatHtml();
      var plain=esc(html).replace(/\n\s*\n/g,'\n'); // text/plain 降级
      var item=new ClipboardItem({
        'text/html':new Blob([html],{type:'text/html'}),
        'text/plain':new Blob([plain],{type:'text/plain'})
      });
      navigator.clipboard.write([item]).then(function(){feedback(true)},function(){feedback(false)})
    }catch(e){
      // 兜底：execCommand + 临时选中
      try{
        var ta=document.createElement('textarea');ta.value=buildWechatHtml();
        ta.style.position='fixed';ta.style.opacity='0';document.body.appendChild(ta);
        ta.select();var ok=document.execCommand('copy');ta.remove();feedback(ok)
      }catch(e2){feedback(false)}
    }
  }
  var bar=document.createElement('div');bar.className='copy-float';
  var btn=document.createElement('button');btn.textContent='复制到公众号';
  btn.title='复制正文 HTML 到剪贴板，粘贴进微信公众号编辑器';
  bar.appendChild(btn);document.body.appendChild(bar);
  // 复制到知乎：直接复制 article 富文本（知乎编辑器宽容，无需内联样式/图片栅格化）
  var btn2=document.createElement('button');btn2.textContent='复制到知乎';
  btn2.onclick=function(){
    var art=document.querySelector('article');
    if(!art){btn2.textContent='无正文';return}
    var item=new ClipboardItem({
      'text/html':new Blob([art.outerHTML],{type:'text/html'}),
      'text/plain':new Blob([art.textContent],{type:'text/plain'})
    });
    navigator.clipboard.write([item]).then(function(){
      btn2.textContent='✅ 已复制';
      setTimeout(function(){btn2.textContent='复制到知乎'},1800);
    },function(){btn2.textContent='❌ 失败'});
  };
  bar.appendChild(btn2);
  var timer=null;
  btn.addEventListener('click',function(){
    writeClip(function(ok){
      btn.textContent=ok?'✅ 已复制':'❌ 复制失败';
      btn.classList.add('done');
      if(timer){clearTimeout(timer)}
      timer=setTimeout(function(){btn.textContent='复制到公众号';btn.classList.remove('done')},1800)
    })
  })
})();
</script>"#;

struct ServeState {
    public: PathBuf,
    /// 构建状态通道：watch 保留最新值，SSE 订阅者连接即收到当前状态。
    status: Arc<watch::Sender<BuildStatus>>,
}

pub fn serve(root: &Path, config: Config, port: u16, open: bool, host: Option<&str>) -> anyhow::Result<()> {
    let root = root.to_path_buf();
    // 首次构建：失败不再中止 serve——错误状态进 watch 通道，浏览器浮层展示，
    // 修复后任一文件保存触发重建并自动恢复。
    // 残留限制：初始失败时（几乎）无产物，直接访问会 404（无注入脚本），
    // 浮层只对已存在的旧产物页面生效。
    let initial = match build::build(&root, &config, true) {
        Ok(_) => BuildStatus { version: 1, error: None },
        Err(e) => {
            eprintln!("⚠️ 初始构建失败（服务器照常启动，修复后自动恢复）:\n{e:#}");
            BuildStatus { version: 1, error: Some(format!("{e:#}")) }
        }
    };
    let status = Arc::new(watch::Sender::new(initial));

    // --open：延迟打开浏览器，等服务器 bind 完成
    if open {
        let url = format!("http://127.0.0.1:{port}");
        std::thread::spawn(move || {
            std::thread::sleep(Duration::from_millis(500));
            open_browser(&url);
        });
    }

    // 文件监听线程（先于 tokio runtime 创建；watch::Sender::send 是同步函数）
    {
        let root = root.clone();
        let config = config.clone();
        let status = status.clone();
        std::thread::spawn(move || {
            if let Err(e) = watch_loop(&root, config, status) {
                eprintln!("文件监听出错: {e}");
            }
        });
    }

    // 定时/周期部署（[deploy.schedule]）：常驻期间后台按计划构建并部署——
    // 定时发布的文章到点自动上线，无需外部 CI cron。
    crate::deploy::spawn_scheduled_deploy(root.clone(), config.clone());

    // HTTP 服务器
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;
    runtime.block_on(async {
        let public = root.join(config.build.output_dir.as_str());
        let state = Arc::new(ServeState { public, status });

        let app = Router::new()
            .route("/__events", get(events_handler))
            .route("/", get(index_handler))
            .route("/{*path}", get(static_handler))
            .with_state(state);

        let host = host.unwrap_or("127.0.0.1");
        let addr = format!("{host}:{port}");
        let listener = tokio::net::TcpListener::bind(&addr).await?;
        let base = format!("http://{}:{port}", if host == "0.0.0.0" { "127.0.0.1" } else { host });
        println!("🚀 开发服务器运行于 {base}  (Ctrl+C 退出)");
        if host == "0.0.0.0"
            && let Some(lan_ip) = lan_ip()
        {
            let lan_url = format!("http://{lan_ip}:{port}");
            println!("📱 局域网预览：{lan_url}（手机同一 Wi-Fi 扫码直达）");
            print_qr(&lan_url);
            println!("⚠️ 0.0.0.0 已向局域网开放此开发服务器，公共网络慎用");
        }
        axum::serve(listener, app).await?;
        Ok::<(), anyhow::Error>(())
    })?;
    Ok(())
}

/// 本机局域网 IP：UDP connect 到公共地址（不实际发包）读出路由选择的本地端点。
fn lan_ip() -> Option<std::net::IpAddr> {
    let sock = std::net::UdpSocket::bind("0.0.0.0:0").ok()?;
    sock.connect("8.8.8.8:80").ok()?;
    sock.local_addr().ok().map(|a| a.ip())
}

/// 终端二维码：Unicode 半块字符渲染（两行并作一行，省屏高）。
fn print_qr(text: &str) {
    let qr = qrcodegen::QrCode::encode_text(text, qrcodegen::QrCodeEcc::Medium)
        .expect("URL 生成二维码失败");
    let size = qr.size() as usize;
    let dark = |x: usize, y: usize| y < size && qr.get_module(x as i32, y as i32);
    println!("┌{}┐", "──".repeat(size + 2));
    for y in (0..size).step_by(2) {
        let mut row = String::from("│  ");
        for x in 0..size {
            let (top, bottom) = (dark(x, y), dark(x, y + 1));
            row.push_str(match (top, bottom) {
                (true, true) => "█",
                (true, false) => "▀",
                (false, true) => "▄",
                (false, false) => " ",
            });
        }
        row.push_str("  │");
        println!("{row}");
    }
    println!("└{}┘", "──".repeat(size + 2));
}

/// SSE：推送构建状态变化（`build` 事件，JSON `{version, error}`）。
///
/// watch 通道在订阅时立即产出当前值；`KeepAlive` 防中间层断开空闲连接。
/// 多行错误文本由 axum 按 SSE 规范拆成多个 `data:` 行，客户端自动还原。
async fn events_handler(
    State(state): State<Arc<ServeState>>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let stream = WatchStream::new(state.status.subscribe()).map(|status| {
        let data = serde_json::to_string(&status).unwrap_or_default();
        Ok(Event::default().event("build").data(data))
    });
    Sse::new(stream).keep_alive(KeepAlive::default())
}

async fn index_handler(
    State(state): State<Arc<ServeState>>,
    headers: HeaderMap,
) -> Response {
    serve_static(&state, "index.html", &headers)
}

async fn static_handler(
    State(state): State<Arc<ServeState>>,
    AxumPath(path): AxumPath<String>,
    headers: HeaderMap,
) -> Response {
    serve_static(&state, &path, &headers)
}

/// 解析 `Accept-Encoding`，返回 `(接受 gzip, 接受 brotli)`。
/// 通配符 `*` 视为仅接受 gzip（保守：br 支持度需显式声明）。
fn accepted_encodings(headers: &HeaderMap) -> (bool, bool) {
    let Some(v) = headers.get(header::ACCEPT_ENCODING).and_then(|v| v.to_str().ok()) else {
        return (false, false);
    };
    let v = v.to_ascii_lowercase();
    let mut gzip = v.contains("gzip");
    let br = v.contains("br");
    if v.contains('*') {
        gzip = true;
    }
    (gzip, br)
}

/// 内存 gzip 压缩（serve 时对注入过脚本的 HTML 实时压缩）。
fn gzip_bytes(data: &[u8]) -> std::io::Result<Vec<u8>> {
    use flate2::write::GzEncoder;
    use flate2::Compression;
    use std::io::Write;
    let mut enc = GzEncoder::new(Vec::new(), Compression::fast());
    enc.write_all(data)?;
    enc.finish()
}

/// 请求相对路径是否安全：所有路径分量都必须是普通名字。
///
/// 拒绝盘符前缀（`C:`）、根目录（`/`）、`..`、`.`——前三者会导致
/// `Path::join` 逃出 `public/` 基目录（Windows 上 `join` 遇绝对路径整体替换）。
fn is_safe_rel_path(rel: &str) -> bool {
    Path::new(rel)
        .components()
        .all(|c| matches!(c, std::path::Component::Normal(_)))
}

fn serve_static(state: &ServeState, rel: &str, headers: &HeaderMap) -> Response {
    // 路径遍历防护：组件级校验。仅挡 `..` 不够——Windows 上 `Path::join`
    // 遇到带盘符/根的路径（如 axum 通配路由解码后保留的 `C:/Windows/win.ini`）
    // 会整体替换基路径，造成任意文件读取。只放行纯相对分量。
    if !is_safe_rel_path(rel) {
        return not_found();
    }
    let mut full = state.public.join(rel);
    if full.is_dir() {
        full = full.join("index.html");
    }

    let (accept_gzip, accept_br) = accepted_encodings(headers);
    match std::fs::read(&full) {
        Ok(data) => {
            let content_type = mime_for(&full);
            if content_type.starts_with("text/html") {
                let html = String::from_utf8_lossy(&data).to_string();
                let injected = inject_reload(&html);
                // 注入过脚本的 HTML 无法用磁盘预压缩产物，实时 gzip
                if accept_gzip
                    && let Ok(compressed) = gzip_bytes(injected.as_bytes())
                {
                    return Response::builder()
                        .header("content-type", content_type)
                        .header("content-encoding", "gzip")
                        .header("vary", "accept-encoding")
                        .body(axum::body::Body::from(compressed))
                        .unwrap();
                }
                return Response::builder()
                    .header("content-type", content_type)
                    .header("vary", "accept-encoding")
                    .body(axum::body::Body::from(injected))
                    .unwrap();
            }
            // 文本类静态文件优先取构建期预压缩产物（.br > .gz），免重复压缩
            if accept_br
                && read_sibling(&full, "br")
                && let Ok(compressed) = std::fs::read(sibling_path(&full, "br"))
            {
                return Response::builder()
                    .header("content-type", content_type)
                    .header("content-encoding", "br")
                    .header("vary", "accept-encoding")
                    .body(axum::body::Body::from(compressed))
                    .unwrap();
            }
            if accept_gzip
                && read_sibling(&full, "gz")
                && let Ok(compressed) = std::fs::read(sibling_path(&full, "gz"))
            {
                return Response::builder()
                    .header("content-type", content_type)
                    .header("content-encoding", "gzip")
                    .header("vary", "accept-encoding")
                    .body(axum::body::Body::from(compressed))
                    .unwrap();
            }
            Response::builder()
                .header("content-type", content_type)
                .header("vary", "accept-encoding")
                .body(axum::body::Body::from(data))
                .unwrap()
        }
        Err(_) => not_found(),
    }
}

/// 输出文本响应；`accept_gzip` 时对正文做实时 gzip（HTML 注入后体积仍可控）。
fn sibling_path(full: &Path, ext: &str) -> PathBuf {
    let mut p = full.as_os_str().to_os_string();
    p.push(format!(".{ext}"));
    PathBuf::from(p)
}

fn read_sibling(full: &Path, ext: &str) -> bool {
    sibling_path(full, ext).is_file()
}

fn not_found() -> Response {
    Response::builder()
        .status(404)
        .header("content-type", "text/plain; charset=utf-8")
        .body(axum::body::Body::from("404 Not Found"))
        .unwrap()
}

fn inject_reload(html: &str) -> String {
    if let Some(pos) = html.rfind("</body>") {
        let mut s = html.to_string();
        s.insert_str(pos, COPY_SCRIPT);
        s.insert_str(pos, LIVE_RELOAD_SCRIPT);
        s
    } else {
        html.to_string()
    }
}

fn mime_for(path: &Path) -> &'static str {
    match path.extension().and_then(|e| e.to_str()) {
        Some("html") | Some("htm") => "text/html; charset=utf-8",
        Some("css") => "text/css; charset=utf-8",
        Some("js") => "application/javascript",
        Some("xml") => "application/xml; charset=utf-8",
        Some("json") => "application/json",
        Some("txt") | Some("text") => "text/plain; charset=utf-8",
        Some("svg") => "image/svg+xml",
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("gif") => "image/gif",
        Some("webp") => "image/webp",
        Some("ico") => "image/x-icon",
        Some("woff") => "font/woff",
        Some("woff2") => "font/woff2",
        Some("webmanifest") => "application/manifest+json",
        _ => "application/octet-stream",
    }
}

fn watch_loop(root: &Path, config: Config, status: Arc<watch::Sender<BuildStatus>>) -> anyhow::Result<()> {
    let mut config = config;
    let (tx, rx) = std::sync::mpsc::channel::<notify::Result<notify::Event>>();

    let mut watcher = notify::recommended_watcher(move |res| {
        let _ = tx.send(res);
    })?;

    for (path, mode) in collect_watch_targets(root) {
        if path.exists() {
            watcher.watch(&path, mode)?;
        }
    }

    // 去抖：记录最后一次事件时间，等事件流稳定 300ms 后再重建。
    // 旧实现"距上次重建不足 300ms 就跳过"会导致编辑器连续保存时永不重建。
    let mut last_event = Instant::now();
    let mut pending = false;
    loop {
        match rx.recv_timeout(Duration::from_millis(100)) {
            Ok(res) => {
                let event = match res {
                    Ok(e) => e,
                    Err(e) => {
                        eprintln!("监听错误: {e}");
                        continue;
                    }
                };
                // 事件类型感知过滤：编辑器临时文件、读访问、rename 旧名事件不触发重建
                if should_rebuild(&event) {
                    last_event = Instant::now();
                    pending = true;
                }
            }
            Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                if pending && last_event.elapsed() >= Duration::from_millis(300) {
                    pending = false;
                    // 每次重建前重读配置：watcher 只上报「文件变了」这一事实，
                    // typall.toml 的语义变更（如切换 [theme] name）必须重新加载
                    // 才会生效——否则热重建永远沿用 serve 启动时的快照。
                    config = reload_config(root, config);
                    println!("🔄 检测到变化，重新构建…");
                    match build::build(root, &config, true) {
                        Ok(_) => {
                            push_status(&status, None);
                            println!("✅ 重建完成，已通知浏览器刷新");
                        }
                        Err(e) => {
                            push_status(&status, Some(format!("{e:#}")));
                            eprintln!("❌ 构建失败:\n{e:#}");
                        }
                    }
                }
            }
            Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => break,
        }
    }
    Ok(())
}

/// 重读项目配置；解析失败时保留原配置并告警（下次重建会再尝试，
/// 构建侧的具体错误也会在浮层/终端暴露，这里不重复打断重建流程）。
fn reload_config(root: &Path, fallback: Config) -> Config {
    match crate::config::Config::load(root) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("⚠️ 配置重读失败，沿用上次配置: {e:#}");
            fallback
        }
    }
}

/// 推送最新构建状态。version 单调递增：失败也递增，否则修复后的版本号
/// 与失败前相同，浏览器判定「未变化」不会刷新。
///
/// `send`（而非 `send_replace`）才会标记 changed 唤醒订阅者；值未变时
/// 不通知——连续两次相同错误只弹一个浮层。无订阅者时返回 Err，忽略。
fn push_status(status: &watch::Sender<BuildStatus>, error: Option<String>) {
    let version = status.borrow().version + 1;
    let _ = status.send(BuildStatus { version, error });
}

/// serve 需要监听的目标清单：内容/资源目录（递归）、站点配置（单文件）、
/// Typst 包缓存（递归——serve 运行中因新增 `#import "@preview/…"` 触发
/// 首次拉包时，包落盘后能自动重建；`.typall/cache`/publish.json 等噪声不监听）。
fn collect_watch_targets(root: &Path) -> Vec<(PathBuf, RecursiveMode)> {
    let mut targets = Vec::new();
    for dir in ["posts", "pages", "assets", "themes"] {
        targets.push((root.join(dir), RecursiveMode::Recursive));
    }
    targets.push((root.join(".typall/packages"), RecursiveMode::Recursive));
    targets.push((root.join("typall.toml"), RecursiveMode::NonRecursive));
    targets
}

/// 一次文件事件是否应触发重建。
///
/// 三层过滤：
/// 1. 读访问（`EventKind::Access`，如搜索索引/杀毒扫描/编辑器打开文件）不算变化；
/// 2. rename 的「旧名」事件（`RenameMode::From`）不算——编辑器原子写是
///    「写临时文件 → rename 覆盖」，旧名事件之后必然跟着新名事件；
/// 3. 事件路径全是编辑器临时文件（swap/锁/备份，见 [`is_temp_artifact`]）不算。
fn should_rebuild(event: &notify::Event) -> bool {
    if matches!(event.kind, EventKind::Access(_)) {
        return false;
    }
    if matches!(
        event.kind,
        EventKind::Modify(ModifyKind::Name(RenameMode::From))
    ) {
        return false;
    }
    event.paths.iter().any(|p| !is_temp_artifact(p))
}

/// 编辑器临时文件判定（不触发重建）：emacs 锁（`.#foo`）、nano/gedit
/// 备份（`#foo#`）、尾波浪备份（`foo~`）、vim swap（`.swp`/`.swx`）。
fn is_temp_artifact(path: &Path) -> bool {
    let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
        return true;
    };
    name.starts_with(".#")
        || (name.starts_with('#') && name.ends_with('#'))
        || name.ends_with('~')
        || name.ends_with(".swp")
        || name.ends_with(".swx")
}

/// 跨平台用系统默认浏览器打开 URL（`serve --open`）。
fn open_browser(url: &str) {
    #[cfg(target_os = "windows")]
    let _ = std::process::Command::new("cmd")
        .args(["/C", "start", "", url])
        .spawn();
    #[cfg(target_os = "macos")]
    let _ = std::process::Command::new("open").arg(url).spawn();
    #[cfg(all(unix, not(target_os = "macos")))]
    let _ = std::process::Command::new("xdg-open").arg(url).spawn();
}

#[cfg(test)]
mod tests {
    use super::*;
    use notify::event::{AccessKind, AccessMode, DataChange};

    #[test]
    fn collect_watch_targets_includes_packages_and_config() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        std::fs::create_dir_all(root.join(".typall/packages")).unwrap();
        std::fs::create_dir_all(root.join("posts")).unwrap();
        let targets = collect_watch_targets(root);
        // 包缓存与配置必须入清单（拉包完成/改配置能触发重建）
        assert!(
            targets
                .iter()
                .any(|(p, m)| *p == root.join(".typall/packages") && *m == RecursiveMode::Recursive)
        );
        assert!(
            targets
                .iter()
                .any(|(p, m)| *p == root.join("typall.toml") && *m == RecursiveMode::NonRecursive)
        );
        // 噪声不入清单：编译缓存与发布状态库
        assert!(!targets.iter().any(|(p, _)| p.ends_with("cache")));
        assert!(!targets.iter().any(|(p, _)| p.ends_with("publish.json")));
    }

    #[test]
    fn should_rebuild_filters_access_and_rename_from() {
        let f = std::path::Path::new("posts/a.typ");
        // 正常写入：触发
        let write = notify::Event::new(EventKind::Modify(ModifyKind::Data(DataChange::Any)))
            .add_path(f.to_owned());
        assert!(should_rebuild(&write));
        // 读访问（索引/杀毒/编辑器打开文件）：不触发
        let access =
            notify::Event::new(EventKind::Access(AccessKind::Close(AccessMode::Any)))
                .add_path(f.to_owned());
        assert!(!should_rebuild(&access));
        // rename 旧名（原子写的前半段）：不触发
        let rename_from = notify::Event::new(EventKind::Modify(ModifyKind::Name(RenameMode::From)))
            .add_path(f.to_owned());
        assert!(!should_rebuild(&rename_from));
        // rename 新名（原子写落地）：触发
        let rename_to = notify::Event::new(EventKind::Modify(ModifyKind::Name(RenameMode::To)))
            .add_path(f.to_owned());
        assert!(should_rebuild(&rename_to));
        // 事件路径全是编辑器临时文件：不触发
        let temp = notify::Event::new(EventKind::Modify(ModifyKind::Data(DataChange::Any)))
            .add_path(std::path::Path::new("posts/.a.typ.swp").to_owned());
        assert!(!should_rebuild(&temp));
        // create 事件（新文章落盘）：触发
        let create = notify::Event::new(notify::EventKind::Create(notify::event::CreateKind::File))
            .add_path(f.to_owned());
        assert!(should_rebuild(&create));
    }

    #[test]
    fn reload_config_picks_up_theme_change() {
        // 回归：热重建曾沿用 serve 启动时的配置快照，
        // 导致运行中切换 [theme] name 不生效（见 docs/THEME-GALLERY.md 实现记录）。
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        std::fs::write(
            root.join("typall.toml"),
            "[site]\ntitle = \"t\"\n\n[theme]\nname = \"default\"\n",
        )
        .unwrap();
        let initial = reload_config(root, Config::load(root).unwrap());
        assert_eq!(initial.theme.name, "default");

        std::fs::write(
            root.join("typall.toml"),
            "[site]\ntitle = \"t\"\n\n[theme]\nname = \"obsidian\"\n",
        )
        .unwrap();
        let reloaded = reload_config(root, initial);
        assert_eq!(reloaded.theme.name, "obsidian");
    }

    #[test]
    fn reload_config_falls_back_on_broken_toml() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        std::fs::write(
            root.join("typall.toml"),
            "[site]\ntitle = \"t\"\n\n[theme]\nname = \"paper\"\n",
        )
        .unwrap();
        let good = Config::load(root).unwrap();
        std::fs::write(root.join("typall.toml"), "not [valid toml").unwrap();
        let fallback = reload_config(root, good);
        assert_eq!(fallback.theme.name, "paper");
    }

    #[test]
    fn accept_encoding_parsing() {
        let mut h = HeaderMap::new();
        h.insert(header::ACCEPT_ENCODING, "gzip, deflate".parse().unwrap());
        assert_eq!(accepted_encodings(&h), (true, false));
        h.insert(header::ACCEPT_ENCODING, "br;q=1.0, gzip;q=0.8".parse().unwrap());
        assert_eq!(accepted_encodings(&h), (true, true));
        // 通配符：保守视为仅 gzip
        h.insert(header::ACCEPT_ENCODING, "*".parse().unwrap());
        assert_eq!(accepted_encodings(&h), (true, false));
        assert_eq!(accepted_encodings(&HeaderMap::new()), (false, false));
    }

    #[test]
    fn gzip_roundtrip() {
        use std::io::Read;
        let mut data = Vec::new();
        for _ in 0..10 {
            data.extend_from_slice(b"hello hello hello hello");
        }
        let compressed = gzip_bytes(&data).unwrap();
        assert!(compressed.len() < data.len(), "重复文本应可压缩");
        let mut out = Vec::new();
        flate2::read::GzDecoder::new(&compressed[..]).read_to_end(&mut out).unwrap();
        assert_eq!(out, data);
    }

    #[test]
    fn is_temp_artifact_filters_editor_noise() {
        assert!(!is_temp_artifact(Path::new("posts/foo.typ")));
        assert!(!is_temp_artifact(Path::new("assets/style.css")));
        assert!(is_temp_artifact(Path::new("posts/.#foo.typ")), "emacs 锁");
        assert!(is_temp_artifact(Path::new("posts/#foo.typ#")), "nano/gedit 备份");
        assert!(is_temp_artifact(Path::new("posts/foo.typ~")), "尾波浪备份");
        assert!(is_temp_artifact(Path::new("posts/.foo.typ.swp")), "vim swap");
        assert!(is_temp_artifact(Path::new("posts/foo.swx")));
    }

    #[test]
    fn watch_status_send_always_wakes_subscribers() {
        let (tx, rx) = watch::channel(BuildStatus { version: 1, error: None });
        // tokio 语义：send 无条件标记 changed（相同值也唤醒；去重须用 send_if_modified）。
        // push_status 因此不依赖值比较，靠 version 自增驱动前端刷新与错误浮层。
        tx.send(BuildStatus { version: 1, error: None }).unwrap();
        assert!(rx.has_changed().unwrap());
        // 值变化同样唤醒，且订阅者读到最新值
        tx.send(BuildStatus { version: 2, error: Some("boom".into()) }).unwrap();
        assert!(rx.has_changed().unwrap());
        assert_eq!(rx.borrow().version, 2);
        assert_eq!(rx.borrow().error.as_deref(), Some("boom"));
    }

    #[test]
    fn inject_reload_includes_both_scripts() {
        let out = inject_reload("<html><body><p>x</p></body></html>");
        assert!(out.contains("EventSource('/__events')"), "应有 Live Reload SSE 脚本");
        assert!(out.contains("复制到公众号"), "应有复制按钮脚本");
        // 两个脚本都注入在 </body> 之前
        assert!(out.find("__typall-build-errors").unwrap() < out.find("</body>").unwrap());
    }

    #[test]
    fn safe_rel_path_blocks_traversal_and_absolute() {
        // 普通相对路径放行
        assert!(is_safe_rel_path("posts/foo/index.html"));
        assert!(is_safe_rel_path("atom.xml"));
        assert!(is_safe_rel_path("目录/图片.svg"));
        // 相对遍历
        assert!(!is_safe_rel_path("../secret"));
        assert!(!is_safe_rel_path("a/../../b"));
        assert!(!is_safe_rel_path(".."));
        // Windows 盘符绝对路径（join 会整体替换基路径，历史漏洞入口）
        assert!(!is_safe_rel_path("C:/Windows/win.ini"));
        assert!(!is_safe_rel_path("C:\\Windows\\win.ini"));
        assert!(!is_safe_rel_path("C:secret"));
        // Unix 根路径
        assert!(!is_safe_rel_path("/etc/passwd"));
        // 当前目录分量
        assert!(!is_safe_rel_path("./x"));
    }

    #[test]
    fn serve_static_rejects_drive_letter_paths() {
        let tmp = std::env::temp_dir().join("typall-serve-traversal-test");
        std::fs::create_dir_all(tmp.join("posts")).unwrap();
        std::fs::write(tmp.join("posts").join("ok.html"), "<p>ok</p>").unwrap();
        let (tx, _rx) = watch::channel(BuildStatus { version: 0, error: None });
        let state = ServeState { public: tmp.clone(), status: Arc::new(tx) };
        let headers = HeaderMap::new();

        // 站内文件可读
        let resp = serve_static(&state, "posts/ok.html", &headers);
        assert_eq!(resp.status(), 200);
        // 盘符绝对路径与遍历一律 404，且确实没有读出 public/ 之外的文件
        for evil in ["C:/Windows/win.ini", "/C:/Windows/win.ini", "../Cargo.toml", "/etc/passwd"] {
            let resp = serve_static(&state, evil, &headers);
            assert_eq!(resp.status(), 404, "路径 `{evil}` 应被拒绝");
        }
        std::fs::remove_dir_all(&tmp).ok();
    }
}
