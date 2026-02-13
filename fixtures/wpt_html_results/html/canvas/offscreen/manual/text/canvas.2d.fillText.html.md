# html/canvas/offscreen/manual/text/canvas.2d.fillText.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/offscreen/manual/text/canvas.2d.fillText.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html class="reftest-wait">
<head>
  <title>HTML 5 Canvas Test: fillText() after transferControlToOffscreen draws text</title>
  <link rel="mismatch" href="empty-ref.html">
  <meta charset="utf-8">
  <meta name="assert" content="fillText() after transferControlToOffscreen() draws text">
  <script src="/common/reftest-wait.js"></script>
</head>
<body onload="runTest()">
<canvas id="c"></canvas>
<script>
  function runTest() {
    const c = document.getElementById("c");
    const ctx = c.transferControlToOffscreen().getContext("2d");
    ctx.fillText("T", 5, 50);
    takeScreenshot();
  }
</script>
</body>
</html>
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
  "source_name": "html/canvas/offscreen/manual/text/canvas.2d.fillText.html"
}
```
