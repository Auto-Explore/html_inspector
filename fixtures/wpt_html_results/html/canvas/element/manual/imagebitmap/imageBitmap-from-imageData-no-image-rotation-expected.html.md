# html/canvas/element/manual/imagebitmap/imageBitmap-from-imageData-no-image-rotation-expected.html

Counts:
- errors: 4
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/manual/imagebitmap/imageBitmap-from-imageData-no-image-rotation-expected.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<html>
<body>
  <canvas id="myCanvas" width="400" height="400"></canvas>
</body>>
<script>
var canvas = document.getElementById('myCanvas');
ctx = canvas.getContext('2d');
image = document.createElement("img");
image.src = "../../../resources/black_white.png"
image.onload = function() {
  Promise.all([
    createImageBitmap(image, { imageOrientation: 'flipY' }),
    ]).then(function(sprites) {
  // Draw image onto the canvas
  ctx.drawImage(sprites[0], 0, 0);
});
}
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.start_tag_without_doctype",
      "message": "Start tag seen without seeing a doctype first. Expected “<!DOCTYPE html>”.",
      "severity": "Error",
      "span": {
        "byte_end": 6,
        "byte_start": 0,
        "col": 1,
        "line": 1
      }
    },
    {
      "category": "Html",
      "code": "html.parser.nonspace_after_body",
      "message": "Non-space character after body.",
      "severity": "Error",
      "span": {
        "byte_end": 82,
        "byte_start": 80,
        "col": 8,
        "line": 4
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_start_tag",
      "message": "Stray start tag “script”.",
      "severity": "Error",
      "span": {
        "byte_end": 90,
        "byte_start": 82,
        "col": 1,
        "line": 5
      }
    },
    {
      "category": "Html",
      "code": "html.parser.end_tag_after_body_closed",
      "message": "Saw an end tag after “body” had been closed.",
      "severity": "Error",
      "span": {
        "byte_end": 479,
        "byte_start": 470,
        "col": 1,
        "line": 18
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
  "source_name": "html/canvas/element/manual/imagebitmap/imageBitmap-from-imageData-no-image-rotation-expected.html"
}
```
