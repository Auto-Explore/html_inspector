# html/canvas/element/manual/imagebitmap/imageBitmapRendering-transferFromImageBitmap-webgl.html

Counts:
- errors: 3
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/manual/imagebitmap/imageBitmapRendering-transferFromImageBitmap-webgl.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel="match" href="imageBitmapRendering-transferFromImageBitmap-webgl-expected.html" />

<body>
  <p>
    Test creating an ImageBitmap from the transferToImageBitmap of a webgl OffscreenCanvas, and then
    transferred to an ImageBitmapRenderingContext.
  </p>
  <canvas id="canvas" width="300" height="300"></canvas>
</body>
<script>

  function drawSquares(ctx) {
    ctx.enable(ctx.SCISSOR_TEST);
    ctx.scissor(0, 150, 150, 150);
    ctx.clearColor(1, 0, 0, 1);
    ctx.clear(ctx.COLOR_BUFFER_BIT);
    ctx.scissor(150, 150, 300, 150);
    ctx.clearColor(0, 1, 0, 1);
    ctx.clear(ctx.COLOR_BUFFER_BIT);
    ctx.scissor(0, 0, 150, 150);
    ctx.clearColor(0, 0, 1, 1);
    ctx.clear(ctx.COLOR_BUFFER_BIT);
  }

  async function runTest() {
    const offscreen = new OffscreenCanvas(300, 300);
    const ctxOffscreen = offscreen.getContext('webgl');
    drawSquares(ctxOffscreen);
    const image = offscreen.transferToImageBitmap();
    const canvas = document.getElementById('canvas');
    const ctx = canvas.getContext('bitmaprenderer');
    ctx.transferFromImageBitmap(image);
  }

  runTest();

</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.stray_start_tag",
      "message": "Stray start tag “script”.",
      "severity": "Error",
      "span": {
        "byte_end": 355,
        "byte_start": 347,
        "col": 1,
        "line": 11
      }
    },
    {
      "category": "Html",
      "code": "html.parser.nonspace_after_body",
      "message": "Non-space character after body.",
      "severity": "Error",
      "span": {
        "byte_end": 1126,
        "byte_start": 355,
        "col": 9,
        "line": 11
      }
    },
    {
      "category": "Html",
      "code": "html.parser.end_tag_after_body_closed",
      "message": "Saw an end tag after “body” had been closed.",
      "severity": "Error",
      "span": {
        "byte_end": 1135,
        "byte_start": 1126,
        "col": 1,
        "line": 38
      }
    },
    {
      "category": "Html",
      "code": "html.head.title.missing",
      "message": "Element “head” is missing a required instance of child element “title”.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/canvas/element/manual/imagebitmap/imageBitmapRendering-transferFromImageBitmap-webgl.html"
}
```
