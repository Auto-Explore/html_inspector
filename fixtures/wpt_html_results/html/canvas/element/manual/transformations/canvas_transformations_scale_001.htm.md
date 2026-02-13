# html/canvas/element/manual/transformations/canvas_transformations_scale_001.htm

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/manual/transformations/canvas_transformations_scale_001.htm",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype HTML>

<html>
    <head>
        <title>HTML5 Canvas Test: scale() transformation</title>
        <link rel="match" href="canvas_transformations_scale_001-ref.htm">
        <link rel="author" title="Microsoft" href="http://www.microsoft.com" />
        <link rel="help" href="http://www.w3.org/TR/2dcontext/#dom-context-2d-scale" />
        <meta name="assert" content="The scale(x, y) method must add the scaling transformation described by the arguments to the transformation matrix." />
        <script type="text/javascript">
            function runTest()
            {
                var canvas = document.getElementById("canvas1");
                var ctx = canvas.getContext("2d");

                // Draw a red rectangle.
                ctx.fillStyle = "rgba(255, 0, 0, 1.0)";
                ctx.fillRect(0, 0, 100, 50);

                // Draw a black rectangle with scaling.
                ctx.fillStyle = "rgba(0, 0, 0, 1.0)";
                ctx.scale(2, 2);
                ctx.fillRect(0, 0, 50, 25);
            }
        </script>
    </head>
    <body onload="runTest()">
        <p>Description: The scale(x, y) method must add the scaling transformation described by the arguments to the transformation matrix.</p>
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
        "byte_end": 539,
        "byte_start": 508,
        "col": 9,
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
  "source_name": "html/canvas/element/manual/transformations/canvas_transformations_scale_001.htm"
}
```
