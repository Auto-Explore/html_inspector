# html/canvas/element/manual/text/canvas.2d.direction.inherit.disconnected.canvas.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/manual/text/canvas.2d.direction.inherit.disconnected.canvas.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype HTML>
<meta charset="utf-8">
<title>HTML5 Canvas Test: The direction attribute inherits from a disconnected canvas element</title>
<link rel="match" href="canvas.2d.direction-ref.html"/>
<link rel="author" href="mailto:schenney@chromium.org"/>
<link rel="help"
  href="https://html.spec.whatwg.org/multipage/canvas.html#text-styles"/>
<meta name="assert" content="Verify that a disconnected canvas with no style uses the canvas direction."/>
<style>
  canvas {
    position: absolute;
    top: 8px;
    left: 8px;
  }
</style>
<canvas id="canvas1" width="300" height="150">
  Browser does not support HTML5 Canvas.
</canvas>
<script type="text/javascript">
  var canvas = document.createElement("canvas");
  canvas.setAttribute("dir", "rtl");
  canvas.setAttribute("width", "300");
  canvas.setAttribute("height", "150");
  var ctx = canvas.getContext("2d");

  // The default for direction is inherit.
  ctx.font = "25px serif";
  ctx.fillText("ABC!", 60, 50);

  document.body.appendChild(canvas);
</script>
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
        "byte_end": 666,
        "byte_start": 635,
        "col": 1,
        "line": 19
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
  "source_name": "html/canvas/element/manual/text/canvas.2d.direction.inherit.disconnected.canvas.html"
}
```
