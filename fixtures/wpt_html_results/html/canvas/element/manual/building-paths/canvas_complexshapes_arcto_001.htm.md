# html/canvas/element/manual/building-paths/canvas_complexshapes_arcto_001.htm

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/manual/building-paths/canvas_complexshapes_arcto_001.htm",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype HTML>
<html>
    <head>
        <title>HTML5 Canvas Test:  arcTo() adds to subpath if same point</title>
        <link rel="match" href="canvas_complexshapes_arcto_001-ref.htm">
        <link rel="author" title="Microsoft" href="http://www.microsoft.com" />
        <link rel="help" href="http://www.w3.org/TR/2dcontext/#dom-context-2d-arcto" />
        <meta name="assert" content="If x1,y1 and x2,y2 are the same point, then arcTo must add x1,y1 to the subpath, and connect that point to x0,y0 with a straight line." />
        <script type="text/javascript">
            function runTest()
            {
                var canvas = document.getElementById("canvas1");
                var ctx = canvas.getContext("2d");
                ctx.moveTo(0, 50);

                // Since (x1, y1) and (x2, y2) are the same point, (x1, y1) must be added to the subpath, thus creating a line.
                ctx.arcTo(100, 50, 100, 50, 10);
                ctx.stroke();
            }
        </script>
    </head>
    <body onload="runTest()">
        <p>Description: If x1,y1 and x2,y2 are the same point, then arcTo must add x1,y1 to the subpath, and connect that point to x0,y0 with a straight line.</p>
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
        "byte_end": 571,
        "byte_start": 540,
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
  "source_name": "html/canvas/element/manual/building-paths/canvas_complexshapes_arcto_001.htm"
}
```
