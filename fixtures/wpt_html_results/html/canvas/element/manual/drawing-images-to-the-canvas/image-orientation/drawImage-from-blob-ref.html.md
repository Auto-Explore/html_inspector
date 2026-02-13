# html/canvas/element/manual/drawing-images-to-the-canvas/image-orientation/drawImage-from-blob-ref.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/manual/drawing-images-to-the-canvas/image-orientation/drawImage-from-blob-ref.html",
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
<title>createImageBitmap and drawImage from a blob with image orientation: from-image, reference</title>
</head>
<body>
  <img id="img-element" style="width: 150px; height: 300px;" src="/css/css-images/image-orientation/support/exif-orientation-8-llo.jpg">
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
        "byte_end": 309,
        "byte_start": 175,
        "col": 3,
        "line": 8
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
  "source_name": "html/canvas/element/manual/drawing-images-to-the-canvas/image-orientation/drawImage-from-blob-ref.html"
}
```
