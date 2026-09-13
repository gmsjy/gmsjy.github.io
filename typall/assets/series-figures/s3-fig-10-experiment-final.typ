// s3-fig-10-experiment-final.typ — 测电源电动势与内阻：装置与 U-I 图线
#import "../series-styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content
  // 电路：电池+滑动变阻器+两表
  wire((1.0, 0.8), (1.0, 4.0))
  wire((1.0, 0.8), (9.0, 0.8))
  wire((9.0, 0.8), (9.0, 4.0))
  wire((1.0, 4.0), (3.4, 4.0))
  wire((6.6, 4.0), (9.0, 4.0))
  circuit-battery((1.0, 2.4), d: (0, 1), len: 1.0)
  wire((1.0, 0.8), (1.0, 1.9))
  wire((1.0, 2.9), (1.0, 4.0))
  content((0.4, 2.4), text(size: 8.5pt, fill: ink, [E, r]), anchor: "east")
  circuit-resistor((5.0, 4.0), d: (1, 0), len: 2.4, label: [滑动变阻器], label-off: (0, 0.5))
  // 电压表并联
  circuit-meter((3.2, 2.4), sym: $V$, r: 0.32)
  wire((3.2, 0.8), (3.2, 2.08))
  wire((3.2, 2.72), (3.2, 4.0))
  circuit-meter((7.0, 2.4), sym: $A$, r: 0.32)
  wire((7.0, 0.8), (7.0, 2.08))
  wire((7.0, 2.72), (7.0, 4.0))
  content((5.0, -0.35), text(size: 8.5pt, fill: gray-line, align(center, [电压表读路端电压 $U$，电流表读干路电流 $I$；多次调节变阻器记录多组 $(U, I)$])))
})
