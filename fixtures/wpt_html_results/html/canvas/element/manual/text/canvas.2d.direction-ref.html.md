# html/canvas/element/manual/text/canvas.2d.direction-ref.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/manual/text/canvas.2d.direction-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype HTML>
<meta charset="utf-8">
<title>HTML5 Canvas Test Reference:  The direction attribute.</title>
<link rel="author" href="mailto:schenney@chromium.org"/>
<script>
  function runTest()
  {
    var canvas = document.getElementById("canvas1");
    var ctx = canvas.getContext("2d");

    ctx.font = "25px serif";
    ctx.direction = "rtl";
    ctx.fillText("ABC!", 60, 50);
  }
</script>
<body onload="runTest()">
  <canvas id="canvas1" width="300" height="150">
    Browser does not support HTML5 Canvas.
  </canvas>
</body>
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
  "source_name": "html/canvas/element/manual/text/canvas.2d.direction-ref.html"
}
```
