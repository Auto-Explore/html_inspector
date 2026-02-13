# html/rendering/replaced-elements/embedded-content-rendering-rules/canvas_scale.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/replaced-elements/embedded-content-rendering-rules/canvas_scale.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<title>Verify that canvases are scaled up to their computed size</title>
<link rel="match" href="canvas_scale_ref.html">
<style>
canvas {
  width: 20px;
  height: 20px;
}
div {
  line-height: 0;
}
</style>
<div><canvas width="16" height="16" data-color="#FF00FF"></canvas><canvas width="16" height="16" data-color="#00FF00"></canvas></div>
<div><canvas width="16" height="16" data-color="#0000FF"></canvas><canvas width="16" height="16" data-color="#FF00FF"></canvas></div>
<script>
var canvases = document.getElementsByTagName('canvas');
for (var i = 0; i < canvases.length; i++) {
  var ctx = canvases[i].getContext('2d');
  ctx.fillStyle = canvases[i].getAttribute('data-color');
  ctx.fillRect(0, 0, 16, 16);
}
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
  "source_name": "html/rendering/replaced-elements/embedded-content-rendering-rules/canvas_scale.html"
}
```
