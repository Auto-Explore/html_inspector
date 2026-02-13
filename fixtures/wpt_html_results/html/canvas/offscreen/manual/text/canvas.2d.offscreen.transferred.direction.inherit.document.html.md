# html/canvas/offscreen/manual/text/canvas.2d.offscreen.transferred.direction.inherit.document.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/offscreen/manual/text/canvas.2d.offscreen.transferred.direction.inherit.document.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype HTML>
<html dir="rtl" class="reftest-wait">
  <meta charset="utf-8">
  <title>HTML5 Canvas Test:  The direction attribute inherits from the document</title>
  <link rel="match" href="canvas.2d.offscreen.direction-ref.html" />
  <link rel="author" href="mailto:schenney@chromium.org"/>
  <link rel="help" href="https://html.spec.whatwg.org/multipage/canvas.html#text-styles"/>
  <meta name="assert" content="An offscreen transferred from a canvas inherits the document dir when the canvas element has none." />
  <script src="/common/reftest-wait.js"></script>
  <script type="text/javascript">
    function runTest()
    {
      var canvas = document.getElementById("canvas1");
      var ctx = canvas.transferControlToOffscreen().getContext("2d");

      // The default for direction is inherit
      ctx.font = "25px serif";
      ctx.fillText("ABC!", 60, 50);

      requestAnimationFrame(() => requestAnimationFrame(() => takeScreenshot()));
    }
  </script>
  <style>
    canvas {
      position: absolute;
      top: 8px;
      left: 8px;
    }
  </style>
  <body onload="runTest()">
    <canvas id="canvas1" width="300" height="150">
      Browser does not support HTML5 Canvas.
    </canvas>
  </body>
</html>

```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 603,
        "byte_start": 572,
        "col": 3,
        "line": 10
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/canvas/offscreen/manual/text/canvas.2d.offscreen.transferred.direction.inherit.document.html"
}
```
