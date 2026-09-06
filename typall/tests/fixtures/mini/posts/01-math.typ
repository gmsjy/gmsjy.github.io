#let title = "微型数学"
#let date = "2026-01-01"
#let tags = ("数学",)
#let series = "M"
#let series_weight = 1
#let draft = false

= 导数

$ f'(x) = lim_(Delta x -> 0) (f(x + Delta x) - f(x))/(Delta x) $ <eq:deriv>

见 @eq:deriv。行内公式 $x^2$ 与 $y = sin x$。

#table(
  columns: (auto, auto),
  [*函数*], [*导数*],
  [$x^n$], [$n x^(n-1)$],
)

> 引用块测试：记忆点。

```python
def deriv(f):
    return f
```
