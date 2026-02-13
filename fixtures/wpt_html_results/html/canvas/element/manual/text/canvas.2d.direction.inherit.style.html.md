# html/canvas/element/manual/text/canvas.2d.direction.inherit.style.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/manual/text/canvas.2d.direction.inherit.style.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype HTML>
<meta charset="utf-8">
<title>HTML5 Canvas Test:  The direction attribute inherits from the canvas style</title>
<link rel="match" href="canvas.2d.direction-ref.html" />
<link rel="author" href="mailto:schenney@chromium.org"/>
<link rel="help" href="https://html.spec.whatwg.org/multipage/canvas.html#text-styles"/>
<meta name="assert" content="When the canvas element has a direction CSS property it should override the dir attribute." />
<script type="text/javascript">
  function runTest() {
    var canvas = document.getElementById("canvas1");
    var ctx = canvas.getContext("2d");

    // The default for direction is inherit
    ctx.font = "25px serif";
    ctx.fillText("ABC!", 60, 50);
  }
</script>
<style>
  canvas {
    direction: rtl;
  }
</style>
<body onload="runTest()">
  <canvas dir="ltr" id="canvas1" width="300" height="150">
    Browser does not support HTML5 Canvas.
  </canvas>
</body>

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
        "byte_end": 487,
        "byte_start": 456,
        "col": 1,
        "line": 8
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
  "source_name": "html/canvas/element/manual/text/canvas.2d.direction.inherit.style.html"
}
```
