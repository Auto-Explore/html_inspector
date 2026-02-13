# html/canvas/element/manual/imagebitmap/imageBitmapRendering-transferFromImageBitmap-flipped.html

Counts:
- errors: 3
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/manual/imagebitmap/imageBitmapRendering-transferFromImageBitmap-flipped.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel="match" href="imageBitmapRendering-transferFromImageBitmap-flipped-expected.html" />
<body>
  <p>Test whether the imageOrientation "flipY" works when creating an ImageBitmap from the ImageData of a canvas, and then transfered to an ImageBitmapRenderingContext.</p>
  <canvas id="canvas" width="300" height="300"></canvas>
</body>
<script>

function drawSquares(ctx) {
  ctx.fillStyle = 'red';
  ctx.fillRect(0,0,150,150);
  ctx.fillStyle = 'green';
  ctx.fillRect(150,0,300,150);
  ctx.fillStyle = 'blue';
  ctx.fillRect(0,150,150,300);
}

async function runTest() {
  const canvas_temp = document.createElement('canvas');
  canvas_temp.width = 300;
  canvas_temp.height = 300;
  const ctx_temp = canvas_temp.getContext('2d');
  drawSquares(ctx_temp);
  const imageSource = ctx_temp.getImageData(0, 0, 300, 300);
  const imageOrientation = 'flipY';
  imageIDFlipped =  await createImageBitmap(imageSource, 0, 0, 300, 300, { imageOrientation });
  const canvas = document.getElementById('canvas');
  const ctx = canvas.getContext('bitmaprenderer');
  ctx.transferFromImageBitmap(imageIDFlipped);
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
        "byte_end": 364,
        "byte_start": 356,
        "col": 1,
        "line": 7
      }
    },
    {
      "category": "Html",
      "code": "html.parser.nonspace_after_body",
      "message": "Non-space character after body.",
      "severity": "Error",
      "span": {
        "byte_end": 1136,
        "byte_start": 364,
        "col": 9,
        "line": 7
      }
    },
    {
      "category": "Html",
      "code": "html.parser.end_tag_after_body_closed",
      "message": "Saw an end tag after “body” had been closed.",
      "severity": "Error",
      "span": {
        "byte_end": 1145,
        "byte_start": 1136,
        "col": 1,
        "line": 34
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
  "source_name": "html/canvas/element/manual/imagebitmap/imageBitmapRendering-transferFromImageBitmap-flipped.html"
}
```
