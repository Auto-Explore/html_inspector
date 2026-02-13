# html/canvas/element/manual/building-paths/canvas_complexshapes_beziercurveto_001-ref.htm

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/manual/building-paths/canvas_complexshapes_beziercurveto_001-ref.htm",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype HTML>
<html>
    <head>
        <title>HTML5 Canvas Test:  bezierCurveTo() must ensure subpaths</title>
        <link rel="author" title="Microsoft" href="http://www.microsoft.com" />
        <script type="text/javascript">
            function runTest()
            {
                var canvas = document.getElementById("canvas1");
                var ctx = canvas.getContext("2d");

                ctx.moveTo(65,25)
                ctx.bezierCurveTo(65, 25, 65, 25, 65, 65);
                ctx.stroke();
                ctx.beginPath();

                ctx.moveTo(35,25)
                ctx.bezierCurveTo(35, 25, 35, 25, 35, 65);
                ctx.stroke();
                ctx.beginPath();

                ctx.moveTo(0,75)
                ctx.bezierCurveTo(0, 75, 50, 150, 100, 75);
                ctx.stroke();
            }
        </script>
    </head>
    <body onload="runTest()">
        <p>Description: bezierCurveTo(cp1x, cp1y, cp2x, cp2y, x, y) must ensure there is a subpath for the point (cp1x,cp1y) if the context has no subpaths, then it must connect the last point in the subpath to the point (x,y).</p>
        <canvas id="canvas1" width="300" height="150">Browser does not support HTML5 Canvas (ref test).</canvas>
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
        "byte_end": 233,
        "byte_start": 202,
        "col": 9,
        "line": 6
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
  "source_name": "html/canvas/element/manual/building-paths/canvas_complexshapes_beziercurveto_001-ref.htm"
}
```
