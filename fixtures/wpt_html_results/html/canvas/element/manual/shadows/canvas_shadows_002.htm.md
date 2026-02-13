# html/canvas/element/manual/shadows/canvas_shadows_002.htm

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/manual/shadows/canvas_shadows_002.htm",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<html>
  <head>
    <title>HTML5 Canvas Test:  Shadows for images</title>
    <link rel="author" title="Microsoft" href="http://www.microsoft.com" />
    <link rel="help" href="http://www.w3.org/TR/2dcontext/#shadows" />
    <link rel="match" href="canvas_shadows_002-ref.htm" />
    <meta name="assert" content="Shadows must be drawn for images." />
    <script type="text/javascript">
      function runTest() {
        var canvas = document.getElementById("canvas1");
        var ctx = canvas.getContext("2d");

        // Draw a red rectangle.
        ctx.fillStyle = "rgba(255, 0, 0, 1.0)";
        ctx.fillRect(150, 0, 100, 50);

        // Set shadow styles to draw a black shadow to overlap the red rectangle.
        ctx.shadowOffsetX = 150;
        ctx.shadowColor = "rgba(0, 0, 0, 1.0)";

        // Draw a black rectangle image on the canvas.
        var img = document.getElementById("imgBlackRect");
        ctx.drawImage(img, 0, 0);
      }
    </script>
  </head>
  <body onload="runTest()">
    <p>Description:  Shadows must be drawn for images.</p>
    <p>Test passes if two black rectangles are shown and there is no red visible on the page.</p>
    <canvas id="canvas1" width="300" height="150">Browser does not support HTML5 Canvas.</canvas>
    <img id="imgBlackRect" style="display:none" width="100" height="50" src="/images/black-rectangle.png">
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
        "byte_end": 402,
        "byte_start": 371,
        "col": 5,
        "line": 9
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 1385,
        "byte_start": 1283,
        "col": 5,
        "line": 32
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
  "source_name": "html/canvas/element/manual/shadows/canvas_shadows_002.htm"
}
```
