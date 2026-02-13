# html/canvas/element/manual/drawing-images-to-the-canvas/image-orientation/drawImage-from-element-swap-width-height-ref.html

Counts:
- errors: 1
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/manual/drawing-images-to-the-canvas/image-orientation/drawImage-from-element-swap-width-height-ref.html",
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
<title>reference drawImage from an element source with image orientation: from-image</title>
<link rel="author" title="Stephen Chenney" href="mailto:schenney@chromium.org">
</head>
<body>
  <img id="img-element" src="/css/css-images/image-orientation/support/exif-orientation-7-rl.jpg">
  <img id="img-element" src="/css/css-images/image-orientation/support/exif-orientation-7-rl.jpg">
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
        "byte_end": 339,
        "byte_start": 243,
        "col": 3,
        "line": 9
      }
    },
    {
      "category": "Html",
      "code": "html.id.duplicate",
      "message": "Duplicate ID “img-element”.",
      "severity": "Error",
      "span": {
        "byte_end": 438,
        "byte_start": 342,
        "col": 3,
        "line": 10
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 438,
        "byte_start": 342,
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
  "source_name": "html/canvas/element/manual/drawing-images-to-the-canvas/image-orientation/drawImage-from-element-swap-width-height-ref.html"
}
```
