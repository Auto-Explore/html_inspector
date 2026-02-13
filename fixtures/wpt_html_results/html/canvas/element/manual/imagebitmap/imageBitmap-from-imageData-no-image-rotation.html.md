# html/canvas/element/manual/imagebitmap/imageBitmap-from-imageData-no-image-rotation.html

Counts:
- errors: 4
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/manual/imagebitmap/imageBitmap-from-imageData-no-image-rotation.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<html>
  <link rel="match" href="imageBitmap-from-imageData-no-image-rotation-expected.html" />
  <style type="text/css">
    canvas {
      image-orientation: none;
    }
  </style>
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
    // The image should be flipped and ignoring "image-orientation" setting
    // in css style.
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
      "code": "html.style.type.unnecessary",
      "message": "The “type” attribute for the “style” element is not needed and should be omitted.",
      "severity": "Warning",
      "span": {
        "byte_end": 121,
        "byte_start": 98,
        "col": 3,
        "line": 3
      }
    },
    {
      "category": "Html",
      "code": "html.parser.nonspace_after_body",
      "message": "Non-space character after body.",
      "severity": "Error",
      "span": {
        "byte_end": 258,
        "byte_start": 256,
        "col": 8,
        "line": 10
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_start_tag",
      "message": "Stray start tag “script”.",
      "severity": "Error",
      "span": {
        "byte_end": 266,
        "byte_start": 258,
        "col": 1,
        "line": 11
      }
    },
    {
      "category": "Html",
      "code": "html.parser.end_tag_after_body_closed",
      "message": "Saw an end tag after “body” had been closed.",
      "severity": "Error",
      "span": {
        "byte_end": 752,
        "byte_start": 743,
        "col": 1,
        "line": 26
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
  "source_name": "html/canvas/element/manual/imagebitmap/imageBitmap-from-imageData-no-image-rotation.html"
}
```
