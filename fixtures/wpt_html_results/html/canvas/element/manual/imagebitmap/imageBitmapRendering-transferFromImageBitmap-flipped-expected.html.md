# html/canvas/element/manual/imagebitmap/imageBitmapRendering-transferFromImageBitmap-flipped-expected.html

Counts:
- errors: 3
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/manual/imagebitmap/imageBitmapRendering-transferFromImageBitmap-flipped-expected.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<body>
  <p>Test whether the imageOrientation "flipY" works when creating an ImageBitmap from the ImageData of a canvas, and then transfered to an ImageBitmapRenderingContext.</p>
  <canvas id="canvas" width="300" height="300"></canvas>
</body>
<script>

function drawSquares(ctx) {
  ctx.fillStyle = 'red';
  ctx.fillRect(0,150,150,150);
  ctx.fillStyle = 'green';
  ctx.fillRect(150,150,300,150);
  ctx.fillStyle = 'blue';
  ctx.fillRect(0,0,150,150);
}

async function runTest() {
  const canvas = document.getElementById('canvas');
  canvas.width = 300;
  canvas.height = 300;
  const ctx = canvas.getContext('2d');
  drawSquares(ctx);
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
        "byte_end": 269,
        "byte_start": 261,
        "col": 1,
        "line": 6
      }
    },
    {
      "category": "Html",
      "code": "html.parser.nonspace_after_body",
      "message": "Non-space character after body.",
      "severity": "Error",
      "span": {
        "byte_end": 671,
        "byte_start": 269,
        "col": 9,
        "line": 6
      }
    },
    {
      "category": "Html",
      "code": "html.parser.end_tag_after_body_closed",
      "message": "Saw an end tag after “body” had been closed.",
      "severity": "Error",
      "span": {
        "byte_end": 680,
        "byte_start": 671,
        "col": 1,
        "line": 27
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
  "source_name": "html/canvas/element/manual/imagebitmap/imageBitmapRendering-transferFromImageBitmap-flipped-expected.html"
}
```
