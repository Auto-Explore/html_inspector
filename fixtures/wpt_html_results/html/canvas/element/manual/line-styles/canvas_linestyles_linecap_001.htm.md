# html/canvas/element/manual/line-styles/canvas_linestyles_linecap_001.htm

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/manual/line-styles/canvas_linestyles_linecap_001.htm",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype HTML>
<html>
    <head>
        <title>HTML5 Canvas Test: "square" lineCap</title>
        <link rel="match" href="canvas_linestyles_linecap_001-ref.htm">
        <link rel="author" title="Microsoft" href="http://www.microsoft.com" />
        <link rel="help" href="http://www.w3.org/TR/2dcontext/#dom-context-2d-linecap" />
        <meta name="assert" content="The square value of lineCap means that a rectangle with the length of the line width and the width of half the line width, placed flat against the edge perpendicular to the direction of the line, must be added at the end of each line." />
        <script type="text/javascript">
            function runTest()
            {
                var canvas = document.getElementById("canvas1");
                var ctx = canvas.getContext("2d");

                // Draw the first red rectangle.
                ctx.fillStyle ="rgba(255, 0, 0, 1.0)";
                ctx.fillRect(75, 0, 25, 50);

                // Draw second red rectangle.
                ctx.fillRect(0, 0, 25, 50);

                // Draw a line with square lineCap.
                ctx.strokeStyle = "rgba(0, 0, 0, 1.0)";
                ctx.lineWidth = 50;
                ctx.lineCap = "square";
                ctx.beginPath();
                ctx.moveTo(25, 25);
                ctx.lineTo(75, 25);
                ctx.stroke();
            }
        </script>
    </head>
    <body onload="runTest()">
        <p>Description: The square value of lineCap means that a rectangle with the length of the line width and the width of half the line width, placed flat against the edge perpendicular to the direction of the line, must be added at the end of each line.</p>
        <canvas id="canvas1" width="300" height="150">Browser does not support HTML5 Canvas.</canvas>
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
        "byte_end": 650,
        "byte_start": 619,
        "col": 9,
        "line": 9
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
  "source_name": "html/canvas/element/manual/line-styles/canvas_linestyles_linecap_001.htm"
}
```
