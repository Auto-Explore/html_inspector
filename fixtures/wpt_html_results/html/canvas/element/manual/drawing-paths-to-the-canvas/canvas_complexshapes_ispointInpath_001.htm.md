# html/canvas/element/manual/drawing-paths-to-the-canvas/canvas_complexshapes_ispointInpath_001.htm

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/manual/drawing-paths-to-the-canvas/canvas_complexshapes_ispointInpath_001.htm",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype HTML>
<html>
    <head>
        <title>HTML5 Canvas Test:  isPointInPath() unaffected by the current transformation matrix</title>
        <script src="/resources/testharness.js"></script>
        <script src="/resources/testharnessreport.js"></script>
        <link rel="author" title="Microsoft" href="http://www.microsoft.com" />
        <link rel="help" href="http://www.w3.org/TR/2dcontext/#dom-context-2d-ispointinpath" />
        <meta name="assert" content="isPointInPath must check the point (x, y) as coordinates unaffected by the current transformation matrix." />
        <script type="text/javascript">
            async_test(function(t) {
              window.addEventListener("load", t.step_func_done(function runTest() {
                var canvas = document.getElementById("canvas1");
                var ctx = canvas.getContext("2d");

                // Create a path that is transformed by a translation transformation matrix.
                ctx.translate(100, 50);
                ctx.rect(0, 0, 100, 50);

                // Ensure that the coordinates passed to isPointInPath are unaffected by the current transformation matrix.
                assert_true(ctx.isPointInPath(125, 75), "isPointInPath(125, 75)");
                assert_false(ctx.isPointInPath(25, 25), "!isPointInPath(25, 25)");
              }));
            }, "isPointInPath unaffected by transformation matrix");
        </script>
    </head>
    <body>
        <p>Description: isPointInPath must check the point (x, y) as coordinates unaffected by the current transformation matrix.</p>
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
        "byte_end": 625,
        "byte_start": 594,
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
  "source_name": "html/canvas/element/manual/drawing-paths-to-the-canvas/canvas_complexshapes_ispointInpath_001.htm"
}
```
