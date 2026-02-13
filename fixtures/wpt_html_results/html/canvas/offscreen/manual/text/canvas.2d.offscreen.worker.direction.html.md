# html/canvas/offscreen/manual/text/canvas.2d.offscreen.worker.direction.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/offscreen/manual/text/canvas.2d.offscreen.worker.direction.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype HTML>
<html class="reftest-wait">
  <meta charset="utf-8">
  <title>HTML5 Canvas Test:  The direction attribute is respected in offscreen worker canvas</title>
  <link rel="match" href="canvas.2d.offscreen.worker.direction-ref.html" />
  <link rel="author" href="mailto:schenney@chromium.org"/>
  <link rel="help" href="https://html.spec.whatwg.org/multipage/canvas.html#text-styles"/>
  <meta name="assert" content="An offscreen canvas in a worker respects the direction text attribute." />
  <script src="/common/reftest-wait.js"></script>
  <script type="text/javascript">
    function runTest() {
      const placeholder_canvas = document.createElement('canvas');
      placeholder_canvas.setAttribute('width', '300');
      placeholder_canvas.setAttribute('height', '150');
      const offscreen = placeholder_canvas.transferControlToOffscreen();

      const worker = new Worker('text-direction-worker.js');
      worker.postMessage({canvas: offscreen}, [offscreen]);

      const canvas = document.getElementById('canvas1');
      const ctx = canvas.getContext('2d', {willReadFrequently: true});
      function checkResult() {
        // Wait until frame propagates.
        ctx.drawImage(placeholder_canvas, 0, 0);
        const pixel = ctx.getImageData(0, 0, 1, 1).data;

        if (pixel[3] == 0) {
          // Result not available, wait longer.
          requestAnimationFrame(checkResult);
        } else {
          takeScreenshot();
        }
      }
      requestAnimationFrame(checkResult);
    }
  </script>
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
        "byte_end": 585,
        "byte_start": 554,
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
  "source_name": "html/canvas/offscreen/manual/text/canvas.2d.offscreen.worker.direction.html"
}
```
