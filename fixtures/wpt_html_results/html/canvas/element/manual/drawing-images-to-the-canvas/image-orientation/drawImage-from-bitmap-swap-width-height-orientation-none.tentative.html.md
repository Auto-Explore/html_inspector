# html/canvas/element/manual/drawing-images-to-the-canvas/image-orientation/drawImage-from-bitmap-swap-width-height-orientation-none.tentative.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/manual/drawing-images-to-the-canvas/image-orientation/drawImage-from-bitmap-swap-width-height-orientation-none.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
<meta charset="utf-8">
<title>createImageBitmap and drawImage from an element source with image orientation: none</title>
<link rel="author" title="Stephen Chenney" href="mailto:schenney@chromium.org">
<link rel="help" href="https://drafts.csswg.org/css-images-3/#propdef-image-orientation">
<link rel="match" href="drawImage-from-bitmap-swap-width-height-orientation-none-ref.html">
  <script>
    image = new Image();
    image.src = "/css/css-images/image-orientation/support/exif-orientation-7-rl.jpg";

    let imageLoadPromise = new Promise(resolve => {
      image.onload = resolve;
    });
    let contentLoadedPromise = new Promise(resolve => {
      window.addEventListener('DOMContentLoaded', resolve);
    });
    Promise.all([imageLoadPromise, contentLoadedPromise]).then( async function() {
      const bitmap = await createImageBitmap(image);
      const can = document.getElementById('bitmap-canvas');
      can.height = 50;
      can.width = 100;
      can.getContext('2d').drawImage(bitmap, 0, 0);
    })
  </script>
</head>
<body>
  <img id="img-element" src="/css/css-images/image-orientation/support/exif-orientation-7-rl.jpg">
  <canvas id="bitmap-canvas" style="image-orientation: none;"></canvas>
</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 1178,
        "byte_start": 1082,
        "col": 3,
        "line": 29
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
  "source_name": "html/canvas/element/manual/drawing-images-to-the-canvas/image-orientation/drawImage-from-bitmap-swap-width-height-orientation-none.tentative.html"
}
```
