# html/rendering/replaced-elements/embedded-content-rendering-rules/canvas-update-with-border-object-fit.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/replaced-elements/embedded-content-rendering-rules/canvas-update-with-border-object-fit.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Verifies canvas with object-fit and border correctly updates</title>
<link rel="match" href="canvas-update-with-border-object-fit-ref.html">
<html class="reftest-wait">
  <div style="width: 300px; height: 100px; background: black; border: 100px solid blue">
    <canvas id="target" width="1000" height="1000"
        style="object-fit: contain; object-position: center; width: 100%; height: 100%">
    </canvas>
  </div>
</html>
<script>
var ctx = target.getContext("2d");
ctx.fillStyle = "red";
ctx.fillRect(0, 0, target.width, target.height);

var x=0, y=0, step=500;
ctx.fillStyle = "green";
function drawRect() {
  ctx.fillRect(x, y, step, step);
  x += step;
  if (x >= target.width) {
    x = 0;
    y += step;
  }
  if (y >= target.height)
    document.documentElement.classList.remove("reftest-wait");
  else
    requestAnimationFrame(drawRect);
}
drawRect();
</script>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/rendering/replaced-elements/embedded-content-rendering-rules/canvas-update-with-border-object-fit.html"
}
```
