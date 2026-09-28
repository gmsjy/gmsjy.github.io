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

use axum::extract::{Path as AxumPath, Query, State};
use axum::http::{header, HeaderMap, StatusCode};
use axum::response::sse::{Event, KeepAlive, Sse};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
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
  // 项目根绝对路径与编辑器 scheme（JSON 字符串字面量；空 EDIT = 未配置编辑器，
  // 错误浮层的 文件:行:列 不做深链）。「编辑本页」按钮由 COPY_SCRIPT 负责。
  var ROOT=__ROOT__,EDIT=__EDITOR_BASE__;
  var es=new EventSource('/__events');
  function fullReload(){location.reload()}
  // 软刷新：fetch 当前页新 HTML，仅替换 main#content 与 <title>，滚动位置不丢。
  // fetch 拿不到可用页面（构建写盘窗口/网络抖动）时先重试一次，仍失败才整页刷新。
  // 错误文本里的 `路径:行:列` 转成编辑器深链（vscode://file/…），逐段用
  // createTextNode 组装——textContent 路线杜绝错误文本注入 HTML。
  var SRC_RE=/([^\s()<>"']+?\.(?:typ|toml|css|html)):([0-9]+):([0-9]+)/g;
  function appendError(text,box){
    if(!EDIT){box.appendChild(document.createTextNode(text));return}
    var last=0,m;
    SRC_RE.lastIndex=0;
    while((m=SRC_RE.exec(text))){
      box.appendChild(document.createTextNode(text.slice(last,m.index)));
      var a=document.createElement('a');
      var p=m[1].replace(/\\/g,'/');
      var abs=/^[A-Za-z]:\//.test(p)||p.charAt(0)==='/';
      a.href=EDIT+(!abs&&ROOT?ROOT+'/':'')+p+':'+m[2]+':'+m[3];
      a.textContent=m[0];
      a.setAttribute('style','color:#ffd7d4;text-decoration:underline');
      box.appendChild(a);
      last=m.index+m[0].length;
    }
    box.appendChild(document.createTextNode(text.slice(last)));
  }
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
      // 通知「复制排版主题」脚本：正文节点已换新，若处于主题预览态需重新套用
      try{document.dispatchEvent(new CustomEvent('typall:content-refresh'))}catch(e){}
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
      item.appendChild(document.createTextNode('⚠ 构建失败\n'));
      appendError(d.error,item);
      box.appendChild(item);
    }else if(v===null){
      v=d.version;
    }else if(d.version!==v){
      softRefresh(d.version);
    }
  });
})();
</script>"#;

/// 「复制到公众号 / 知乎」浮动按钮 + 排版主题选择器 + 剪贴板脚本
/// （仅 serve 时注入，build 产物零 JS）。
///
/// **复制排版主题**（`src/copy_theme.rs` 内置 5 套，`__COPY_THEMES_JSON__`
/// 注入时替换为 JSON）：
/// - 浮动栏下拉切换主题；选中后把该主题的内联样式**直接套到当前正文**上
///   （所见即所得——页面呈现的就是粘贴进平台的效果），选回「站点样式」还原；
/// - 选择存 localStorage，刷新/重开自动恢复；软刷新换入新正文后自动重套
///   （监听 Live Reload 脚本派发的 `typall:content-refresh` 事件）；
/// - 两个复制按钮都按当前主题出稿；「站点样式」档位下按默认主题（墨理蓝）
///   出稿，与历史写死行为一致。
///
/// 复制时实时处理 `<article>` 的**克隆副本**（预览态套用的样式也不受影响，
/// 永远从干净副本出稿，不同主题反复切换不会叠加）：
///   1. 公式/插图 `<svg>` 的 `<defs><symbol id>` + `<use xlink:href>` 结构
///      **use/symbol 展开**成纯 `<path>`（删光 id/defs/xlink）——绕开公众号
///      保存层删 id/defs 导致公式空白的问题，同时保留矢量。
///   2. 普通 `<img src="相对路径">` 转绝对 URL（依赖 site.url 正确解析页面 URL）。
///   3. 按主题把逐元素内联 style 写进正文（公众号剥类名与外链 CSS，只认内联）；
///      块级公式的 flex 预览布局改为「公式居中 + 编号右对齐独立行」。
///   4. 双格式写入剪贴板（text/html + text/plain），平台编辑器吃 HTML 富文本。
const COPY_SCRIPT: &str = r#"<style>
.copy-float{position:fixed;right:16px;top:40%;z-index:9999;display:flex;flex-direction:column;gap:8px;align-items:flex-end}
.copy-float button,.copy-float select{font:14px/1.4 system-ui,sans-serif;padding:8px 12px;border:1px solid #ddd;border-radius:8px;background:#fff;color:#333;cursor:pointer;box-shadow:0 2px 8px rgba(0,0,0,.08);transition:.15s}
.copy-float select{max-width:132px;padding:7px 8px}
.copy-float button:hover{border-color:#07c160;color:#07c160}
.copy-float button.done{border-color:#07c160;background:#07c160;color:#fff}
@media (max-width:640px){.copy-float{right:8px;top:auto;bottom:12px;flex-direction:row;align-items:center}}
</style>
<script>
(function(){
  var THEMES=__COPY_THEMES_JSON__;
  // 项目根绝对路径 / 本页源文件（项目根相对）/ 编辑器 scheme 前缀
  // （JSON 字符串字面量；SRC 或 EDIT 为空串 = 无源文件映射或未配置编辑器）。
  var ROOT=__ROOT__,SRC=__SOURCE__,EDIT=__EDITOR_BASE__;
  if(!document.querySelector('article')){return}
  var SITE='__site__';
  var LS='typallCopyTheme';
  function themeById(id){
    for(var i=0;i<THEMES.length;i++){if(THEMES[i].id===id)return THEMES[i]}
    return null
  }
  // 当前档位：默认「站点样式」（预览原样），localStorage 记住上次选择
  var cur=SITE;
  try{var saved=localStorage.getItem(LS);if(saved&&themeById(saved)){cur=saved}}catch(e){}

  function esc(s){return String(s).replace(/&/g,'&amp;').replace(/</g,'&lt;').replace(/>/g,'&gt;')}
  // use/symbol 展开：返回处理后的 svg，传入的 DOM 不变
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
  // —— 主题应用：styles 槽位（标签名 / article / codeInline / codeInPre /
  //    eqBlock / eqSvg / eqNum）→ 元素内联 style。合并进已有 style，不覆盖 svg。
  function styleOf(el,css){
    var pre=el.getAttribute('style'); el.setAttribute('style',(pre?pre.replace(/;?$/,';'):'')+css)
  }
  function applyStyles(root,S){
    ['h1','h2','h3','h4','p','blockquote','pre','ul','ol','li','table','th','td','hr','a','img','strong','em','del','svg'].forEach(function(t){
      if(!S[t]){return}
      Array.prototype.forEach.call(root.querySelectorAll(t),function(el){styleOf(el,S[t])})
    });
    if(S.article){styleOf(root,S.article)}
    // 行内 code 与 pre 内 code 的衬底/字体（公众号保留 font-family）
    Array.prototype.forEach.call(root.querySelectorAll('code'),function(el){
      if(el.closest('pre')){if(S.codeInPre)styleOf(el,S.codeInPre)}
      else if(S.codeInline){styleOf(el,S.codeInline)}
    });
    // 块级公式：flex/绝对定位(预览用)公众号不支持 → 「公式居中 + 编号单独右对齐行」。
    // 容器 text-align:center 使行内 svg 居中；eq-num 变 display:block 落到公式下一行靠右。
    Array.prototype.forEach.call(root.querySelectorAll('div[data-equation="block"]'),function(d){
      if(S.eqBlock)styleOf(d,S.eqBlock);
      Array.prototype.forEach.call(d.querySelectorAll(':scope > svg'),function(sv){
        var st=sv.getAttribute('style')||'';
        sv.setAttribute('style',st.replace(/display\s*:\s*block\s*;?/i,'')+(S.eqSvg||''))
      });
      Array.prototype.forEach.call(d.querySelectorAll(':scope > .eq-num'),function(sp){
        if(S.eqNum)styleOf(sp,S.eqNum)
      })
    });
  }
  // 站点导航/装饰元素不入剪贴板（复制时剥离；预览态保留以便站内跳转）
  function stripChrome(root){
    Array.prototype.forEach.call(root.querySelectorAll('nav.toc,.prev,.next,.meta,.toc,link,script,style'),function(e){e.remove()})
  }
  // 干净副本：永远从服务器原始 DOM 克隆，主题反复切换/复制不叠加样式
  var pristine=null;
  function capture(){var a=document.querySelector('article');if(a){pristine=a.cloneNode(true)}}
  // 从干净副本构建主题化正文（strip=true 时剥离站内导航，复制用）
  function buildStyled(theme,strip){
    var art=pristine.cloneNode(true);
    if(strip){stripChrome(art)}
    // use/symbol 展开（SVG 保矢量；依赖 id/defs 的引用实体化）
    Array.prototype.forEach.call(art.querySelectorAll('svg'),function(s){
      var ex=expandSvg(s); s.parentNode.replaceChild(ex,s)
    });
    absolutize(art);
    applyStyles(art,theme.styles);
    return art
  }
  // 当前出稿主题：「站点样式」档位下按默认主题（与历史写死行为一致）
  function currentTheme(){return cur===SITE?THEMES[0]:themeById(cur)}
  // 所见即所得：把主题样式直接套到页面正文；「站点样式」档位还原原始 DOM
  function applyPreview(){
    var live=document.querySelector('article');
    if(!live||!pristine){return}
    if(cur===SITE){
      live.parentNode.replaceChild(pristine.cloneNode(true),live)
    }else{
      live.parentNode.replaceChild(buildStyled(themeById(cur),false),live)
    }
  }
  capture();
  // Live Reload 软刷新换入新正文后：重新捕获干净副本并重套当前主题
  document.addEventListener('typall:content-refresh',function(){
    capture();
    if(cur!==SITE){applyPreview()}
  });
  if(cur!==SITE){applyPreview()}
  // 复制：按当前主题出稿（公众号 / 知乎共用管线；知乎编辑器较宽容，
  // 多余的内联样式会被其清洗，带上无副作用）
  function buildCopyHtml(){
    var art=buildStyled(currentTheme(),true);
    return art.outerHTML
  }
  function writeClip(done){
    try{
      var html=buildCopyHtml();
      var plain=esc(html).replace(/\n\s*\n/g,'\n'); // text/plain 降级
      var item=new ClipboardItem({
        'text/html':new Blob([html],{type:'text/html'}),
        'text/plain':new Blob([plain],{type:'text/plain'})
      });
      navigator.clipboard.write([item]).then(function(){done(true)},function(){done(false)})
    }catch(e){
      // 兜底：execCommand + 临时选中
      try{
        var ta=document.createElement('textarea');ta.value=buildCopyHtml();
        ta.style.position='fixed';ta.style.opacity='0';document.body.appendChild(ta);
        ta.select();var ok=document.execCommand('copy');ta.remove();done(ok)
      }catch(e2){done(false)}
    }
  }
  var bar=document.createElement('div');bar.className='copy-float';
  // 排版主题选择器：选中即所见即所得套用到正文
  var sel=document.createElement('select');
  sel.title='复制排版主题：选中后正文即时套用该风格，复制按此主题出稿';
  var optSite=document.createElement('option');
  optSite.value=SITE;optSite.textContent='站点样式';sel.appendChild(optSite);
  THEMES.forEach(function(t){
    var o=document.createElement('option');o.value=t.id;o.textContent=t.label;sel.appendChild(o)
  });
  sel.value=cur;
  sel.onchange=function(){
    cur=sel.value;
    try{localStorage.setItem(LS,cur)}catch(e){}
    applyPreview()
  };
  bar.appendChild(sel);
  // ✏️ 编辑本页：用配置的编辑器 scheme 打开本页对应的 .typ 源文件
  if(SRC&&EDIT){
    var eb=document.createElement('button');eb.textContent='✏️ 编辑本页';
    eb.title='在编辑器中打开 '+SRC;
    eb.onclick=function(){location.href=EDIT+ROOT+'/'+SRC};
    bar.appendChild(eb);
  }
  var btn=document.createElement('button');btn.textContent='复制到公众号';
  btn.title='按当前主题复制正文富文本，粘贴进微信公众号编辑器';
  btn.addEventListener('click',function(){
    writeClip(function(ok){
      btn.textContent=ok?'✅ 已复制':'❌ 复制失败';
      btn.classList.toggle('done',ok);
      setTimeout(function(){btn.textContent='复制到公众号';btn.classList.remove('done')},1800)
    })
  });
  bar.appendChild(btn);
  var btn2=document.createElement('button');btn2.textContent='复制到知乎';
  btn2.title='按当前主题复制正文富文本，粘贴进知乎文章编辑器';
  btn2.addEventListener('click',function(){
    writeClip(function(ok){
      btn2.textContent=ok?'✅ 已复制':'❌ 复制失败';
      btn2.classList.toggle('done',ok);
      setTimeout(function(){btn2.textContent='复制到知乎';btn2.classList.remove('done')},1800)
    })
  });
  bar.appendChild(btn2);
  document.body.appendChild(bar);
})();
</script>"#;

struct ServeState {
    public: PathBuf,
    /// 构建状态通道：watch 保留最新值，SSE 订阅者连接即收到当前状态。
    status: Arc<watch::Sender<BuildStatus>>,
    /// 复制排版主题 JSON（内置 + `copythemes/` 自定义）。自定义主题文件
    /// 保存触发重建后由 watch 线程刷新，HTML 响应注入时读取最新值。
    copy_themes: Arc<std::sync::RwLock<String>>,
    /// 项目根（「编辑本页」源文件反推、上传落盘、素材库列表的基准）。
    root: PathBuf,
    /// 项目根绝对路径（`\`→`/`，无尾分隔符），编辑器深链用。
    root_url: String,
    /// 编辑器 scheme 前缀（如 `vscode://file/`），空 = 未配置编辑器。
    editor_base: String,
}

pub fn serve(
    root: &Path,
    config: Config,
    port: u16,
    open: bool,
    host: Option<&str>,
    focus: Option<PathBuf>,
) -> anyhow::Result<()> {
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

    // 复制排版主题（内置 + copythemes/ 自定义）初始加载；后续随重建热刷新
    let copy_themes = Arc::new(std::sync::RwLock::new(crate::copy_theme::themes_json_for(&root)));

    // 构建互斥锁：文件监听重建、live 单篇重建、定时部署构建三个 actor 都会跑
    // build::build——并发时 manifest 读写与孤儿清理会互相踩踏（A 的清理可能
    // 删掉 B 刚写出的文件）。一把项目级锁把构建/部署串行化。
    let build_lock = Arc::new(std::sync::Mutex::new(()));

    // 实时模式状态：目标文章 + 单篇编译器 + 文章列表（导航上下文）。
    // 初始化失败降级为常规 serve（编辑器修好文章后全量重建路径仍可用）。
    let live = match &focus {
        Some(path) => {
            let focus_slug = crate::content::rel_path(&root, path)
                .trim_end_matches(".typ")
                .replace('\\', "/");
            match build_live_state(&root, &config, path, &focus_slug) {
                Ok(state) => {
                    println!("✍️  实时模式：监听 {focus_slug}（保存即单篇重编译，毫秒级刷新）");
                    Some(state)
                }
                Err(e) => {
                    eprintln!("⚠️ 实时模式初始化失败，退回常规 serve：{e:#}");
                    None
                }
            }
        }
        None => None,
    };
    let focus_slug = live.as_ref().map(|s| s.focus_slug.clone());

    // --open 或实时模式：延迟打开浏览器，等服务器 bind 完成（实时模式直达文章页）
    if open || focus.is_some() {
        let url = match &focus_slug {
            Some(slug) => format!("http://127.0.0.1:{port}/{slug}/"),
            None => format!("http://127.0.0.1:{port}"),
        };
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
        let build_lock = build_lock.clone();
        let copy_themes = copy_themes.clone();
        std::thread::spawn(move || {
            if let Err(e) = watch_loop(&root, config, status, live, build_lock, copy_themes) {
                eprintln!("文件监听出错: {e}");
            }
        });
    }

    // 定时/周期部署（[deploy.schedule]）：常驻期间后台按计划构建并部署——
    // 定时发布的文章到点自动上线，无需外部 CI cron。
    crate::deploy::spawn_scheduled_deploy(root.clone(), config.clone(), build_lock);

    // HTTP 服务器
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;
    runtime.block_on(async {
        let public = root.join(config.build.output_dir.as_str());
        let state = Arc::new(ServeState {
            public,
            status,
            copy_themes,
            root: root.clone(),
            root_url: root_url(&root),
            editor_base: editor_base_for(&config.serve.editor),
        });

        let app = Router::new()
            .route("/__events", get(events_handler))
            .route("/__gallery", get(gallery_handler))
            .route("/__upload", post(upload_handler))
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

async fn index_handler(State(state): State<Arc<ServeState>>, headers: HeaderMap) -> Response {
    serve_static_async(state, "index.html".to_string(), headers).await
}

async fn static_handler(
    State(state): State<Arc<ServeState>>,
    AxumPath(path): AxumPath<String>,
    headers: HeaderMap,
) -> Response {
    serve_static_async(state, path, headers).await
}

/// 一次 HTML 响应的注入上下文（每请求构建）。
struct Inject {
    /// 复制排版主题 JSON（内置 + 自定义）。
    themes_json: String,
    /// 项目根绝对路径（`\`→`/`，无尾分隔符），编辑器深链用。
    root: String,
    /// 编辑器 scheme 前缀（如 `vscode://file/`），空 = 未配置编辑器。
    editor_base: String,
    /// 本页源 `.typ`（项目根相对），空 = 无法反推（集合页/静态资源等）。
    source: String,
}

/// 磁盘读 + 实时 gzip 都是阻塞操作，必须移入 spawn_blocking：
/// LAN 模式（--host 0.0.0.0）下并发请求共享同一 tokio worker 池，
/// 在 async 上下文里做慢盘读会卡住整个 runtime。
async fn serve_static_async(state: Arc<ServeState>, rel: String, headers: HeaderMap) -> Response {
    let themes = state
        .copy_themes
        .read()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .clone();
    let inject = Inject {
        themes_json: themes,
        root: state.root_url.clone(),
        editor_base: state.editor_base.clone(),
        source: derive_source(&state.root, &rel).unwrap_or_default(),
    };
    match tokio::task::spawn_blocking(move || serve_static(&state.public, &rel, &headers, &inject))
        .await
    {
        Ok(resp) => resp,
        Err(e) => {
            eprintln!("静态文件请求处理失败: {e}");
            not_found()
        }
    }
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
    // 盘符与反斜杠显式拒绝：`Component::Prefix` 只在 Windows 存在，若不加
    // 这条，Linux 上 "C:/Windows/win.ini" 的分量全是 Normal，守卫会被绕过
    !rel.contains(':')
        && !rel.contains('\\')
        && Path::new(rel)
            .components()
            .all(|c| matches!(c, std::path::Component::Normal(_)))
}

fn serve_static(public: &Path, rel: &str, headers: &HeaderMap, inject: &Inject) -> Response {
    // 路径遍历防护：组件级校验。仅挡 `..` 不够——Windows 上 `Path::join`
    // 遇到带盘符/根的路径（如 axum 通配路由解码后保留的 `C:/Windows/win.ini`）
    // 会整体替换基路径，造成任意文件读取。只放行纯相对分量。
    if !is_safe_rel_path(rel) {
        return not_found();
    }
    let mut full = public.join(rel);
    if full.is_dir() {
        full = full.join("index.html");
    }

    let (accept_gzip, accept_br) = accepted_encodings(headers);
    match std::fs::read(&full) {
        Ok(data) => {
            let content_type = mime_for(&full);
            if content_type.starts_with("text/html") {
                let html = String::from_utf8_lossy(&data).to_string();
                let injected = inject_reload(&html, inject);
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

fn inject_reload(html: &str, inject: &Inject) -> String {
    if let Some(pos) = html.rfind("</body>") {
        let mut s = html.to_string();
        s.insert_str(pos, &copy_script(inject));
        s.insert_str(pos, &live_script(inject));
        s
    } else {
        html.to_string()
    }
}

/// 复制按钮脚本：占位符注入。替换顺序刻意把 `__COPY_THEMES_JSON__` 放在
/// 最后——主题 JSON 来自用户文件，先替换其余占位符可避免用户数据被二次扫描。
fn copy_script(inject: &Inject) -> String {
    COPY_SCRIPT
        .replace("__ROOT__", &js_str(&inject.root))
        .replace("__SOURCE__", &js_str(&inject.source))
        .replace("__EDITOR_BASE__", &js_str(&inject.editor_base))
        .replace("__COPY_THEMES_JSON__", &inject.themes_json)
}

/// Live Reload 脚本：根路径 + 编辑器前缀（构建错误浮层的 `文件:行:列` 深链）。
fn live_script(inject: &Inject) -> String {
    LIVE_RELOAD_SCRIPT
        .replace("__ROOT__", &js_str(&inject.root))
        .replace("__EDITOR_BASE__", &js_str(&inject.editor_base))
}

/// Rust 字符串 → 安全的 JS/JSON 字符串字面量（双引号、转义齐备）。
fn js_str(s: &str) -> String {
    serde_json::to_string(s).unwrap_or_else(|_| "\"\"".into())
}

/// `[serve] editor` 配置 → 深链前缀。`vscode` → `vscode://file/`；
/// 已含 `://` 的值原样使用（任意自定义 scheme）；空 → 空串（不注入编辑入口）。
fn editor_base_for(editor: &str) -> String {
    let e = editor.trim();
    if e.is_empty() {
        return String::new();
    }
    if e.contains("://") {
        e.to_string()
    } else {
        format!("{e}://file/")
    }
}

/// 项目根绝对路径规范形（`\`→`/`，无尾分隔符）——vscode 深链要求正斜杠。
fn root_url(root: &Path) -> String {
    let s = root.to_string_lossy().replace('\\', "/");
    s.trim_end_matches('/').to_string()
}

/// 由产物相对路径反推源 `.typ`（项目根相对），供「✏️ 编辑本页」。
/// `posts/foo` 或 `posts/foo/index.html` → `posts/foo.typ`；未知 slug 依次
/// 探测 `pages/<slug>.typ`；集合页/分页/静态资源无源文件 → None。
fn derive_source(root: &Path, rel: &str) -> Option<String> {
    let mut stem = rel.replace('\\', "/");
    if let Some(s) = stem.strip_suffix("index.html") {
        stem = s.to_string();
    }
    let stem = stem.trim_end_matches('/');
    if stem.is_empty() {
        return None;
    }
    [
        format!("{stem}.typ"),
        format!("pages/{stem}.typ"),
    ]
    .into_iter()
    .find(|c| root.join(c).is_file())
}

// ────────────────────────── 素材库（/__gallery）与上传（/__upload） ──────────────────────────

/// 上传大小上限。截图/照片足够；防误传大文件阻塞内存。
const MAX_UPLOAD_BYTES: usize = 20 * 1024 * 1024;

/// 允许落盘的图片扩展名（白名单，防任意文件写入）。
const IMAGE_EXTS: &[&str] = &["png", "jpg", "jpeg", "svg", "webp", "gif", "avif"];

#[derive(serde::Deserialize)]
struct UploadQuery {
    filename: Option<String>,
}

/// 素材库页：拖拽/粘贴上传 + 已有图片 + 成品图/figkit 片段速查。
/// 独立响应，不走 inject_reload（无 Live Reload/复制按钮注入）。
async fn gallery_handler(State(state): State<Arc<ServeState>>) -> Response {
    let root = state.root.clone();
    // 目录扫描（151 个成品图的 readdir）按仓库惯例放阻塞线程
    let html = tokio::task::spawn_blocking(move || build_gallery_html(&root)).await;
    match html {
        Ok(h) => ([(header::CONTENT_TYPE, "text/html; charset=utf-8")], h).into_response(),
        Err(e) => {
            eprintln!("素材库页生成失败: {e}");
            not_found()
        }
    }
}

/// 图片上传：`POST /__upload?filename=xx.png`，body 为原始字节。
/// 净化文件名 + 扩展名白名单 + 大小上限 + 重名自动加序号，落 `assets/images/`。
async fn upload_handler(
    State(state): State<Arc<ServeState>>,
    Query(q): Query<UploadQuery>,
    body: axum::body::Bytes,
) -> Response {
    let root = state.root.clone();
    let filename = q.filename.clone();
    let result =
        tokio::task::spawn_blocking(move || save_upload(&root, filename.as_deref(), &body)).await;
    match result {
        Ok(Ok((name, snippet))) => Json(serde_json::json!({
            "ok": true, "name": name, "snippet": snippet,
        }))
        .into_response(),
        Ok(Err(e)) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "ok": false, "error": e.to_string() })),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "ok": false, "error": e.to_string() })),
        )
            .into_response(),
    }
}

/// 上传落盘：返回 (最终文件名, 可粘贴的 typst 片段)。
fn save_upload(root: &Path, raw: Option<&str>, bytes: &[u8]) -> anyhow::Result<(String, String)> {
    if bytes.is_empty() {
        anyhow::bail!("上传内容为空");
    }
    if bytes.len() > MAX_UPLOAD_BYTES {
        anyhow::bail!("图片超过 20MB 上限");
    }
    let Some(name) = raw.and_then(sanitize_upload_name) else {
        anyhow::bail!("文件名缺失或扩展名不受支持（png/jpg/jpeg/svg/webp/gif/avif）");
    };
    let dir = root.join("assets").join("images");
    std::fs::create_dir_all(&dir)?;
    let path = unique_path(&dir, &name);
    std::fs::write(&path, bytes)?;
    let final_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(&name)
        .to_string();
    Ok((
        final_name.clone(),
        format!("#image(\"../assets/images/{final_name}\")"),
    ))
}

/// 上传文件名净化：取最后路径分量，仅保留 文字数字（含 CJK）/`-`/`_`/`.`，
/// 其余折叠为 `-`，首尾折叠符剔除。扩展名必须在白名单内（小写比较）。
fn sanitize_upload_name(raw: &str) -> Option<String> {
    let base = raw.rsplit(['/', '\\']).next()?.trim();
    let (stem, ext) = base.rsplit_once('.')?;
    if stem.is_empty() {
        return None;
    }
    let ext = ext.to_ascii_lowercase();
    if !IMAGE_EXTS.contains(&ext.as_str()) {
        return None;
    }
    let mut clean = String::new();
    let mut last_dash = true;
    for c in stem.chars() {
        if c.is_alphanumeric() || c == '-' || c == '_' || c == '.' {
            clean.push(c);
            last_dash = false;
        } else if !last_dash {
            clean.push('-');
            last_dash = true;
        }
    }
    let clean = clean.trim_matches('-').to_string();
    if clean.is_empty() {
        return None;
    }
    Some(format!("{clean}.{ext}"))
}

/// 目标名已存在时自动加序号：`foo.png` → `foo-2.png` → `foo-3.png`…
fn unique_path(dir: &Path, name: &str) -> PathBuf {
    let cand = dir.join(name);
    if !cand.exists() {
        return cand;
    }
    let (stem, ext) = name.rsplit_once('.').unwrap_or((name, ""));
    for n in 2.. {
        let p = dir.join(format!("{stem}-{n}.{ext}"));
        if !p.exists() {
            return p;
        }
    }
    unreachable!("序号穷尽不可能")
}

/// assets/images/ 下的图片文件名（白名单扩展名，字典序）。
fn list_images(root: &Path) -> Vec<String> {
    let Ok(entries) = std::fs::read_dir(root.join("assets").join("images")) else {
        return Vec::new();
    };
    let mut names: Vec<String> = entries
        .filter_map(|e| e.ok())
        .map(|e| e.file_name().to_string_lossy().to_string())
        .filter(|n| {
            n.rsplit('.')
                .next()
                .map(|e| IMAGE_EXTS.contains(&e.to_ascii_lowercase().as_str()))
                .unwrap_or(false)
        })
        .collect();
    names.sort();
    names
}

/// assets/series-figures/ 下的成品图 stem（.typ，字典序）。
fn list_series_figures(root: &Path) -> Vec<String> {
    let Ok(entries) = std::fs::read_dir(root.join("assets").join("series-figures")) else {
        return Vec::new();
    };
    let mut stems: Vec<String> = entries
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|x| x.to_str()) == Some("typ"))
        .filter_map(|p| p.file_stem().and_then(|x| x.to_str()).map(String::from))
        .collect();
    stems.sort();
    stems
}

/// 成品图文件名 → typst 导入别名。`s1-fig-01-difficulty` → `fig-difficulty`、
/// `s1-fig-01b-matrix` → `fig-matrix`（剥系列前缀与编号段）；不匹配约定时
/// 以文件名兜底（非法字符折叠为 `-`）。
fn fig_alias(stem: &str) -> String {
    // 剥系列前缀 `s<数字>-`
    let mut name = stem;
    if let Some(rest) = name.strip_prefix('s') {
        let digits = rest.len() - rest.trim_start_matches(|c: char| c.is_ascii_digit()).len();
        if digits > 0 && rest[digits..].starts_with('-') {
            name = &rest[digits + 1..];
        }
    }
    // 剥编号段 `01-` / `01b-`
    let core = name.strip_prefix("fig-").unwrap_or(name);
    let chars: Vec<char> = core.chars().collect();
    let mut i = 0;
    while i < chars.len() && chars[i].is_ascii_digit() {
        i += 1;
    }
    if i > 0 && i < chars.len() && chars[i].is_ascii_lowercase() {
        i += 1;
    }
    if i > 0 && i < chars.len() && chars[i] == '-' {
        i += 1;
    }
    let core: String = if i == 0 { core.to_string() } else { chars[i..].iter().collect() };
    let core = if core.is_empty() { name.to_string() } else { core };
    let safe: String = core
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() || c == '-' || c == '_' { c } else { '-' })
        .collect();
    format!("fig-{}", safe.trim_matches('-'))
}

fn esc_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

/// figkit 六类模板速查（示例摘自 posts/fig-kit 讲解文）。
const FIGKIT_SNIPPETS: &[(&str, &str)] = &[
    (
        "plot · 函数曲线图",
        "#plot(-1, 5, -1, 4, curves: ((f: x => x * x, x0: 0, x1: 2.1, color: phys-orange)))",
    ),
    (
        "flow · 流程框图",
        "#flow(((x: 0, y: 1, t: [步骤一]), (x: 0, y: 0, t: [步骤二])), links: ((a: 0, b: 1, label: [箭头])))",
    ),
    (
        "cycle · 转化三角",
        "#cycle(([$\"A\"$], [$\"B\"$], [$\"C\"$]), edges: ((0, 1, label: [转化]), (1, 2, label: [转化]), (2, 0, label: [回归])))",
    ),
    (
        "card · 全景卡",
        "#card([卡片标题], ([条目一], [条目二], [条目三]))",
    ),
    (
        "number-line · 数轴区间",
        "#number-line(-4, 4, step: 1, intervals: (((x: -1, open: true), (x: 3))), points: ((2.5, \"solid\")))",
    ),
    (
        "raw + 零件 · 装置图",
        "#raw(size: 0.7cm, { beaker(-1.4, -1.8, 2.4, 2.3, liquid: 0.6) /* + electrode/salt-bridge/导线… */ })",
    ),
];

fn build_gallery_html(root: &Path) -> String {
    // 已有图片网格
    let images = list_images(root);
    let imgs_html = if images.is_empty() {
        r#"<li class="empty">（暂无图片——上传后会出现在这里）</li>"#.to_string()
    } else {
        images
            .iter()
            .map(|n| {
                let snip = format!("#image(\"../assets/images/{n}\")");
                format!(
                    "<li class=\"img-card\"><img loading=\"lazy\" src=\"/assets/images/{}\" alt=\"{}\">\
                     <div class=\"row\"><span class=\"name\" title=\"{}\">{}</span>\
                     <button data-snippet=\"{}\">复制片段</button></div></li>",
                    esc_html(n),
                    esc_html(n),
                    esc_html(n),
                    esc_html(n),
                    esc_html(&snip),
                )
            })
            .collect::<Vec<_>>()
            .join("")
    };
    // 成品图 import 片段
    let figs_html = match list_series_figures(root) {
        figs if figs.is_empty() => {
            r#"<li class="empty">（assets/series-figures/ 不存在）</li>"#.to_string()
        }
        figs => figs
            .iter()
            .map(|stem| {
                let import = format!(
                    "#import \"../assets/series-figures/{stem}.typ\": fig as {}",
                    fig_alias(stem)
                );
                format!(
                    "<li><span class=\"name\" title=\"{}\">{}</span>\
                     <button data-snippet=\"{}\">复制 import</button></li>",
                    esc_html(stem),
                    esc_html(stem),
                    esc_html(&import),
                )
            })
            .collect::<Vec<_>>()
            .join(""),
    };
    // figkit 速查
    let kit_html = FIGKIT_SNIPPETS
        .iter()
        .map(|(label, snip)| {
            format!(
                "<li><span class=\"name\">{}</span><button data-snippet=\"{}\">复制片段</button></li>",
                esc_html(label),
                esc_html(snip),
            )
        })
        .collect::<Vec<_>>()
        .join("");

    GALLERY_HTML
        .replace("__IMAGES__", &imgs_html)
        .replace("__FIGURES__", &figs_html)
        .replace("__FIGKIT__", &kit_html)
}

const GALLERY_HTML: &str = r##"<!doctype html>
<html lang="zh-CN">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width,initial-scale=1">
<title>素材库 · typall</title>
<style>
body{font:14px/1.6 system-ui,"PingFang SC","Microsoft YaHei",sans-serif;color:#20222a;margin:24px auto;max-width:960px;padding:0 16px;background:#faf9f6}
h1{font-size:22px;margin:0 0 4px} h2{font-size:17px;margin:28px 0 10px;border-bottom:1px solid #e6e2d6;padding-bottom:4px}
.hint{color:#686a72;font-size:13px;margin:4px 0 14px}
#drop{border:2px dashed #c9c4b4;border-radius:10px;padding:26px;text-align:center;color:#686a72;cursor:pointer;transition:.15s;background:#fff}
#drop.hover{border-color:#07c160;background:#f0faf3;color:#07c160}
#out{margin-top:12px}
.result{background:#f0faf3;border:1px solid #bfe6cc;border-radius:8px;padding:8px 12px;margin:8px 0;display:flex;gap:8px;align-items:center;flex-wrap:wrap}
.result code{background:#fff;padding:2px 6px;border-radius:4px;font-size:12.5px;flex:1;min-width:200px;overflow-x:auto;white-space:nowrap}
button{font:13px/1.4 system-ui,sans-serif;padding:5px 10px;border:1px solid #ddd;border-radius:6px;background:#fff;color:#333;cursor:pointer}
button:hover{border-color:#07c160;color:#07c160}
ul{list-style:none;padding:0;margin:0;display:grid;grid-template-columns:repeat(auto-fill,minmax(280px,1fr));gap:8px}
li{background:#fff;border:1px solid #e6e2d6;border-radius:8px;padding:8px 10px;display:flex;align-items:center;gap:8px;min-width:0}
li.empty{grid-column:1/-1;color:#686a72;border-style:dashed;justify-content:center}
.img-card{flex-direction:column;align-items:stretch;padding:8px}
.img-card img{width:100%;height:130px;object-fit:contain;background:#f3f1ec;border-radius:6px}
.row{display:flex;align-items:center;gap:8px;margin-top:6px;min-width:0}
.name{flex:1;min-width:0;overflow:hidden;text-overflow:ellipsis;white-space:nowrap;font-size:12.5px;color:#686a72}
#q{font:14px system-ui;padding:6px 10px;border:1px solid #ddd;border-radius:6px;width:240px;margin-bottom:10px}
#flash{position:fixed;bottom:18px;left:50%;transform:translateX(-50%);background:#20222a;color:#fff;padding:8px 18px;border-radius:20px;font-size:13px;opacity:0;transition:.25s;pointer-events:none}
#flash.show{opacity:.92}
code{font-family:SFMono-Regular,Consolas,monospace}
</style>
</head>
<body>
<h1>素材库</h1>
<p class="hint">拖拽 / Ctrl+V 粘贴 / 点击下方区域上传图片 → 存入 <code>assets/images/</code>，生成可直接粘贴进文章的调用片段。</p>
<div id="drop">把图片拖到这里，或点击选择文件；截图后直接 Ctrl+V 也行</div>
<input id="file" type="file" accept="image/*" multiple hidden>
<div id="out"></div>

<h2>已有图片（assets/images/）</h2>
<ul id="imgs">__IMAGES__</ul>

<h2>成品图片段（assets/series-figures/）</h2>
<p class="hint">点击复制 #import 行；调用方式见各文章的 figure-block 用法。</p>
<input id="q" type="search" placeholder="按文件名过滤…">
<ul id="figs">__FIGURES__</ul>

<h2>figkit 模板速查</h2>
<p class="hint">需先 <code>#import "../assets/figkit.typ": *</code>；参数详解见《figkit 插图模板库》一文。</p>
<ul id="kit">__FIGKIT__</ul>

<div id="flash"></div>
<script>
(function(){
  var drop=document.getElementById('drop'),file=document.getElementById('file'),
      out=document.getElementById('out'),flashEl=document.getElementById('flash'),
      q=document.getElementById('q');
  function flash(t){flashEl.textContent=t;flashEl.classList.add('show');
    setTimeout(function(){flashEl.classList.remove('show')},1600)}
  function copyText(t){
    function fallback(){var ta=document.createElement('textarea');ta.value=t;
      ta.style.position='fixed';ta.style.opacity='0';document.body.appendChild(ta);
      ta.select();try{document.execCommand('copy');flash('✅ 已复制')}catch(e){flash('❌ 复制失败')}ta.remove()}
    if(navigator.clipboard&&navigator.clipboard.writeText){
      navigator.clipboard.writeText(t).then(function(){flash('✅ 已复制')},fallback)
    }else{fallback()}
  }
  document.addEventListener('click',function(e){
    var b=e.target&&e.target.closest&&e.target.closest('[data-snippet]');
    if(b){copyText(b.getAttribute('data-snippet'))}
  });
  // 过滤成品图列表
  q.addEventListener('input',function(){
    var kw=q.value.toLowerCase(),lis=document.querySelectorAll('#figs li:not(.empty)');
    for(var i=0;i<lis.length;i++){
      lis[i].style.display=lis[i].textContent.toLowerCase().indexOf(kw)>=0?'':'none';
    }
  });
  // 上传
  function uploadFile(f){
    var r=new FileReader();
    r.onload=function(){
      fetch('/__upload?filename='+encodeURIComponent(f.name),{method:'POST',body:r.result})
      .then(function(res){return res.json()})
      .then(function(j){
        if(!j.ok){flash('❌ '+(j.error||'上传失败'));return}
        var div=document.createElement('div');div.className='result';
        var code=document.createElement('code');code.textContent=j.snippet;
        var btn=document.createElement('button');btn.textContent='复制片段';
        btn.setAttribute('data-snippet',j.snippet);
        var span=document.createElement('span');span.textContent='✅ '+j.name;
        span.style.cssText='font-size:12.5px;color:#686a72';
        div.appendChild(span);div.appendChild(code);div.appendChild(btn);
        out.insertBefore(div,out.firstChild);
        flash('✅ 已存为 assets/images/'+j.name+'，片段已就绪');
      }).catch(function(){flash('❌ 上传失败')});
    };
    r.readAsArrayBuffer(f);
  }
  drop.addEventListener('click',function(){file.click()});
  file.addEventListener('change',function(){for(var i=0;i<file.files.length;i++)uploadFile(file.files[i]);file.value=''});
  ['dragover','dragenter'].forEach(function(ev){drop.addEventListener(ev,function(e){e.preventDefault();drop.classList.add('hover')})});
  ['dragleave','dragend'].forEach(function(ev){drop.addEventListener(ev,function(){drop.classList.remove('hover')})});
  drop.addEventListener('drop',function(e){
    e.preventDefault();drop.classList.remove('hover');
    for(var i=0;i<e.dataTransfer.files.length;i++)uploadFile(e.dataTransfer.files[i]);
  });
  document.addEventListener('paste',function(e){
    var items=e.clipboardData&&e.clipboardData.items;if(!items)return;
    for(var i=0;i<items.length;i++){
      if(items[i].type.indexOf('image')===0){
        var f=items[i].getAsFile();if(!f)continue;
        var ext=(items[i].type.split('/')[1]||'png').replace('jpeg','jpg');
        uploadFile(new File([f],'paste-'+Date.now()+'.'+ext));
      }
    }
  });
})();
</script>
</body>
</html>
"##;

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

fn watch_loop(
    root: &Path,
    config: Config,
    status: Arc<watch::Sender<BuildStatus>>,
    mut live: Option<LiveState>,
    build_lock: Arc<std::sync::Mutex<()>>,
    copy_themes: Arc<std::sync::RwLock<String>>,
) -> anyhow::Result<()> {
    // 复制主题 JSON 随每次重建刷新（copythemes/ 新增/修改/删除均走重建路径，
    // 重扫目录代价可忽略）；失败也不影响下一轮。
    let refresh_copy_themes = || {
        *copy_themes.write().unwrap_or_else(std::sync::PoisonError::into_inner) =
            crate::copy_theme::themes_json_for(root);
    };

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

    // 去抖：记录最后一次事件时间与涉及路径，等事件流稳定 300ms 后再重建。
    // 旧实现"距上次重建不足 300ms 就跳过"会导致编辑器连续保存时永不重建。
    let mut last_event = Instant::now();
    let mut pending = false;
    let mut pending_paths: Vec<PathBuf> = Vec::new();
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
                    pending_paths
                        .extend(event.paths.iter().filter(|p| !is_temp_artifact(p)).cloned());
                }
            }
            Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                if pending && last_event.elapsed() >= Duration::from_millis(300) {
                    pending = false;
                    let paths = std::mem::take(&mut pending_paths);
                    // 每次重建前重读配置：watcher 只上报「文件变了」这一事实，
                    // typall.toml 的语义变更（如切换 [theme] name）必须重新加载
                    // 才会生效——否则热重建永远沿用 serve 启动时的快照。
                    config = reload_config(root, config);
                    // 与定时部署/live 快速路径互斥：构建产物（manifest、孤儿清理）
                    // 不允许并发写（见 serve() 里 build_lock 的注释）。
                    let _guard =
                        build_lock.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                    // 实时模式快速路径：本次变化只涉及目标文章 → 单篇重编译，
                    // 跳过集合页/Feed/搜索索引等站点级产物（毫秒级出页面）。
                    if let Some(state) = live.as_mut()
                        && !paths.is_empty()
                        && paths.iter().all(|p| is_same_file(p, &state.focus_canon))
                    {
                        println!("⚡ 检测到文章修改，单篇实时重编译…");
                        match live_rebuild_article(root, &config, state) {
                            Ok(()) => {
                                push_status(&status, None);
                                refresh_copy_themes();
                                println!("✅ 实时预览已更新，已通知浏览器刷新");
                            }
                            Err(e) => {
                                push_status(&status, Some(format!("{e:#}")));
                                eprintln!("❌ 实时重编译失败:\n{e:#}");
                            }
                        }
                        continue;
                    }
                    println!("🔄 检测到变化，重新构建…");
                    match build::build(root, &config, true) {
                        Ok(_) => {
                            push_status(&status, None);
                            refresh_copy_themes();
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

/// 实时模式状态：目标文章、单篇编译器与文章列表（供上一篇/下一篇/专栏导航）。
struct LiveState {
    focus_slug: String,
    focus_path: PathBuf,
    /// 规范化后的目标路径：事件路径 canonicalize 后与之比较。
    focus_canon: PathBuf,
    compiler: crate::compile::LiveCompiler,
    /// 日期倒序（与 build 同口径）；写作模式全量可见（含草稿/定时文章）。
    posts: Vec<crate::ir::CompiledDoc>,
}

fn build_live_state(
    root: &Path,
    config: &Config,
    focus_path: &Path,
    focus_slug: &str,
) -> anyhow::Result<LiveState> {
    let compiler = crate::compile::LiveCompiler::new(root, config)?;
    // 文章列表走编译缓存（全部命中，毫秒级），供导航上下文；失败降级空列表，
    // 目标文章保存后仍能单篇重编译（此时 prev/next 暂缺）。
    let mut posts = match crate::compile::compile_documents(root, config) {
        Ok(docs) => docs.posts,
        Err(e) => {
            eprintln!("⚠️ 文章列表初始化失败（导航暂缺）：{e:#}");
            Vec::new()
        }
    };
    posts.sort_by(|a, b| b.meta.date.cmp(&a.meta.date));
    let focus_canon = std::fs::canonicalize(focus_path).unwrap_or_else(|_| focus_path.to_path_buf());
    Ok(LiveState {
        focus_slug: focus_slug.to_string(),
        focus_path: focus_path.to_path_buf(),
        focus_canon,
        compiler,
        posts,
    })
}

/// 事件路径是否即目标文章（canonicalize 比较，失败退回字面相等）。
fn is_same_file(a: &Path, canon_b: &Path) -> bool {
    match std::fs::canonicalize(a) {
        Ok(canon) => canon == canon_b,
        Err(_) => a == canon_b,
    }
}

/// 实时快速路径：只重编译目标文章并重渲染其页面。集合页 / Feed / 搜索索引 /
/// 分享卡等站点级产物不在快速路径内，由下次全量重建统一刷新。
fn live_rebuild_article(root: &Path, config: &Config, state: &mut LiveState) -> anyhow::Result<()> {
    let t0 = Instant::now();
    let doc = state.compiler.compile(root, &state.focus_path, config)?;
    let slug = doc.slug.clone();
    match state.posts.iter().position(|p| p.slug == slug) {
        Some(i) => state.posts[i] = doc,
        None => state.posts.push(doc),
    }
    state.posts.sort_by(|a, b| b.meta.date.cmp(&a.meta.date));
    let i = state.posts.iter().position(|p| p.slug == slug).expect("上方刚插入");
    let theme = crate::theme::Theme::load(root, config)?;
    let nav = crate::site::nav_links(&state.posts);
    let series_navs = crate::site::series_nav_index(&state.posts)?;
    let prev = state.posts.get(i + 1).map(|p| (p.slug.clone(), crate::site::post_title(p)));
    let next = if i > 0 {
        state.posts.get(i - 1).map(|p| (p.slug.clone(), crate::site::post_title(p)))
    } else {
        None
    };
    let prev_ref = prev.as_ref().map(|(s, t)| (s.as_str(), t.as_str()));
    let next_ref = next.as_ref().map(|(s, t)| (s.as_str(), t.as_str()));
    let html = crate::site::render_article(
        &theme,
        config,
        &state.posts[i],
        &nav,
        prev_ref,
        next_ref,
        series_navs[i].as_ref(),
    )?;
    let mut writer = crate::writer::SiteWriter::new(config.output_dir(root));
    writer.write_str(&format!("{slug}/index.html"), &html)?;
    println!("   耗时 {}ms", t0.elapsed().as_millis());
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
    for dir in ["posts", "pages", "assets", "themes", crate::copy_theme::CUSTOM_DIR] {
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

    /// 测试用内置主题 JSON（生产路径由 ServeState 传入 themes_json_for）。
    fn builtin_json() -> String {
        serde_json::to_string(&crate::copy_theme::builtin()).unwrap()
    }

    /// 测试用注入上下文：root/source/editor 默认空（未配置编辑器）。
    fn test_inject(themes: &str) -> Inject {
        Inject {
            themes_json: themes.to_string(),
            root: String::new(),
            editor_base: String::new(),
            source: String::new(),
        }
    }

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
        let themes = builtin_json();
        let out = inject_reload("<html><body><p>x</p></body></html>", &test_inject(&themes));
        assert!(out.contains("EventSource('/__events')"), "应有 Live Reload SSE 脚本");
        assert!(out.contains("复制到公众号"), "应有复制按钮脚本");
        // 两个脚本都注入在 </body> 之前
        assert!(out.find("__typall-build-errors").unwrap() < out.find("</body>").unwrap());
    }

    #[test]
    fn copy_script_carries_themes_and_dispatch_hook() {
        let out = inject_reload(
            "<html><body><p>x</p></body></html>",
            &test_inject(&builtin_json()),
        );
        // 占位符已替换为主题 JSON（含默认主题与暗色主题标记）
        assert!(!out.contains("__COPY_THEMES_JSON__"), "主题 JSON 占位符必须被替换");
        assert!(out.contains("\"id\":\"moli\""), "应含默认主题墨理蓝");
        assert!(out.contains("\"dark\":true"), "应含暗色主题标记");
        // 软刷新事件挂钩：Live Reload 换入新正文后复制脚本重套主题
        assert!(out.contains("typall:content-refresh"));
    }

    #[test]
    fn inject_reload_passes_custom_themes_through() {
        // 自定义主题 JSON 原样进入注入脚本（热加载通道存在性的锚点）
        let custom = r#"[{"id":"academy","label":"学院青","dark":false,"styles":{"p":"color:#123;"}},{"id":"moli","label":"墨理蓝（默认）","dark":false,"styles":{}}]"#;
        let out = inject_reload("<html><body><p>x</p></body></html>", &test_inject(custom));
        assert!(out.contains("\"id\":\"academy\""), "自定义主题应随注入下发");
        assert!(out.contains("学院青"));
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
        let headers = HeaderMap::new();

        let themes = builtin_json();

        // 站内文件可读
        let resp = serve_static(&tmp, "posts/ok.html", &headers, &test_inject(&themes));
        assert_eq!(resp.status(), 200);
        // 盘符绝对路径与遍历一律 404，且确实没有读出 public/ 之外的文件
        for evil in ["C:/Windows/win.ini", "/C:/Windows/win.ini", "../Cargo.toml", "/etc/passwd"] {
            let resp = serve_static(&tmp, evil, &headers, &test_inject(&themes));
            assert_eq!(resp.status(), 404, "路径 `{evil}` 应被拒绝");
        }
        std::fs::remove_dir_all(&tmp).ok();
    }

    #[test]
    fn editor_base_for_resolves_schemes() {
        assert_eq!(editor_base_for(""), "", "未配置 = 空前缀");
        assert_eq!(editor_base_for("  "), "");
        assert_eq!(editor_base_for("vscode"), "vscode://file/");
        assert_eq!(editor_base_for("cursor"), "cursor://file/");
        // 自定义 scheme（已含 ://）原样使用
        assert_eq!(editor_base_for("windsurf://file/"), "windsurf://file/");
    }

    #[test]
    fn inject_scripts_carry_editor_and_source_placeholders_replaced() {
        let inject = Inject {
            themes_json: builtin_json(),
            root: "E:/site".into(),
            editor_base: "vscode://file/".into(),
            source: "posts/hello.typ".into(),
        };
        let out = inject_reload("<html><body><p>x</p></body></html>", &inject);
        // 占位符全部被替换（JSON 字符串字面量形式）
        for ph in ["__ROOT__", "__SOURCE__", "__EDITOR_BASE__", "__COPY_THEMES_JSON__"] {
            assert!(!out.contains(ph), "占位符 {ph} 必须被替换");
        }
        assert!(out.contains("\"E:/site\""), "根路径应以 JSON 字符串注入");
        assert!(out.contains("编辑本页"), "应含编辑本页按钮");
        assert!(out.contains("SRC_RE"), "错误浮层应含源码位置深链解析");
    }

    #[test]
    fn derive_source_maps_output_to_typ() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(tmp.path().join("posts")).unwrap();
        std::fs::create_dir_all(tmp.path().join("pages")).unwrap();
        std::fs::write(tmp.path().join("posts").join("hello.typ"), "").unwrap();
        std::fs::write(tmp.path().join("pages").join("about.typ"), "").unwrap();
        assert_eq!(
            derive_source(tmp.path(), "posts/hello/index.html").as_deref(),
            Some("posts/hello.typ")
        );
        assert_eq!(
            derive_source(tmp.path(), "posts/hello").as_deref(),
            Some("posts/hello.typ"),
            "无尾斜杠的目录请求同样映射"
        );
        assert_eq!(
            derive_source(tmp.path(), "about/index.html").as_deref(),
            Some("pages/about.typ"),
            "独立页回落 pages/ 探测"
        );
        assert_eq!(derive_source(tmp.path(), "index.html"), None, "首页无源文件");
        assert_eq!(derive_source(tmp.path(), "tags/chem/index.html"), None, "集合页无源文件");
    }

    #[test]
    fn sanitize_upload_name_filters_paths_and_extensions() {
        // 常规名保留
        assert_eq!(
            sanitize_upload_name("cell.png").as_deref(),
            Some("cell.png")
        );
        // CJK 与空格折叠
        assert_eq!(
            sanitize_upload_name("细胞 结构 01.png").as_deref(),
            Some("细胞-结构-01.png")
        );
        // 路径分量剥离（含 Windows 反斜杠）
        assert_eq!(
            sanitize_upload_name(r"..\..\evil\img.jpg").as_deref(),
            Some("img.jpg")
        );
        // 扩展名白名单
        assert_eq!(sanitize_upload_name("a.exe"), None);
        assert_eq!(sanitize_upload_name("noext"), None);
        assert_eq!(sanitize_upload_name(""), None);
        assert_eq!(sanitize_upload_name(".png"), None, "空 stem 拒绝");
        // 折叠符收尾剔除
        assert_eq!(sanitize_upload_name("a!!b.png").as_deref(), Some("a-b.png"));
    }

    #[test]
    fn save_upload_writes_and_makes_snippet() {
        let tmp = tempfile::tempdir().unwrap();
        let (name, snippet) =
            save_upload(tmp.path(), Some("细 胞.png"), b"\x89PNG fake").unwrap();
        assert_eq!(name, "细-胞.png");
        assert!(tmp.path().join("assets/images/细-胞.png").is_file());
        assert_eq!(snippet, "#image(\"../assets/images/细-胞.png\")");
        // 重名自动加序号
        let (name2, _) = save_upload(tmp.path(), Some("细 胞.png"), b"x").unwrap();
        assert_eq!(name2, "细-胞-2.png");
        // 空文件与超限拒绝
        assert!(save_upload(tmp.path(), Some("a.png"), b"").is_err());
    }

    #[test]
    fn fig_alias_strips_series_and_number_segments() {
        assert_eq!(fig_alias("s1-fig-01-difficulty"), "fig-difficulty");
        assert_eq!(fig_alias("s1-fig-01b-matrix"), "fig-matrix");
        assert_eq!(fig_alias("s12-fig-04-number-system"), "fig-number-system");
        assert_eq!(fig_alias("plain-name"), "fig-plain-name", "不匹配约定以原名兜底");
    }

    #[test]
    fn gallery_html_lists_images_and_figures() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(tmp.path().join("assets/images")).unwrap();
        std::fs::create_dir_all(tmp.path().join("assets/series-figures")).unwrap();
        std::fs::write(tmp.path().join("assets/images/shot.png"), b"x").unwrap();
        std::fs::write(tmp.path().join("assets/images/evil.html"), b"x").unwrap();
        std::fs::write(tmp.path().join("assets/series-figures/s1-fig-01-difficulty.typ"), "").unwrap();
        let html = build_gallery_html(tmp.path());
        assert!(html.contains("/assets/images/shot.png"), "图片网格应列出白名单文件");
        assert!(!html.contains("evil.html"), "非白名单文件不得出现");
        assert!(html.contains("fig as fig-difficulty"), "成品图应给出推导别名");
        assert!(html.contains("复制片段"), "应含复制按钮");
        assert!(!html.contains("__IMAGES__"), "占位符必须全部替换");
    }
}
