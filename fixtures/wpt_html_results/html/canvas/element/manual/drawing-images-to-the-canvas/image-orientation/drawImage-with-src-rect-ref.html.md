# html/canvas/element/manual/drawing-images-to-the-canvas/image-orientation/drawImage-with-src-rect-ref.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/manual/drawing-images-to-the-canvas/image-orientation/drawImage-with-src-rect-ref.html",
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
<title>reference for drawImage with image orientation: from-image and a sub-image source rect</title>
<link rel="author" title="Stephen Chenney" href="mailto:schenney@chromium.org">
  <script>
    window.onload = () => {
      const img = document.getElementById('img-element');

      const can = document.getElementById('bitmap-canvas');
      can.height = img.height;
      can.width = img.width;
      can.getContext('2d').drawImage(img, 40, 20, 50, 25, 0, 0, can.width, can.height);
    };
  </script>
</head>
<body>
  <img id="img-element" src="/css/css-images/image-orientation/support/exif-orientation-3-lr-pre-rotated.jpg">
  <canvas id="bitmap-canvas"></canvas>
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
        "byte_end": 685,
        "byte_start": 577,
        "col": 3,
        "line": 19
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
  "source_name": "html/canvas/element/manual/drawing-images-to-the-canvas/image-orientation/drawImage-with-src-rect-ref.html"
}
```
