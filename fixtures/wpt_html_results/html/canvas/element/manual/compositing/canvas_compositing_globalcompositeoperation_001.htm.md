# html/canvas/element/manual/compositing/canvas_compositing_globalcompositeoperation_001.htm

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/manual/compositing/canvas_compositing_globalcompositeoperation_001.htm",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype HTML>
<html>
    <head>
        <title>HTML5 Canvas Test:  globalCompositeOperation "destination-over"</title>
        <link rel="match" href="canvas_compositing_globalcompositeoperation_001-ref.htm">
        <link rel="author" title="Microsoft" href="http://www.microsoft.com" />
        <link rel="help" href="http://www.w3.org/TR/2dcontext/#dom-context-2d-globalcompositeoperation" />
        <meta name="assert" content="If the globalCompositeOperation is set to 'destination-over', display the destination image wherever the destination image is opaque." />
        <script type="text/javascript">
            function runTest()
            {
                var canvas = document.getElementById("canvas1");
                var ctx = canvas.getContext("2d");

                // Source image.
                ctx.fillStyle = "rgba(0, 0, 0, 1.0)";
                ctx.fillRect(0, 0, 100, 50);

                // Assign the globalCompositeOperation.
                ctx.globalCompositeOperation = "destination-over";

                // Destination image.
                ctx.fillStyle = "rgba(255, 0, 0, 1.0)";
                ctx.fillRect(0, 0, 100, 50);
            }
        </script>
    </head>
    <body onload="runTest()">
        <p>Description: If the globalCompositeOperation is set to "destination-over", display the destination image wherever the destination image is opaque.</p>
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
        "byte_end": 612,
        "byte_start": 581,
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
  "source_name": "html/canvas/element/manual/compositing/canvas_compositing_globalcompositeoperation_001.htm"
}
```
