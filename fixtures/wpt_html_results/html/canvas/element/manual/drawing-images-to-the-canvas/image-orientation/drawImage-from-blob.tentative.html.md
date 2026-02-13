# html/canvas/element/manual/drawing-images-to-the-canvas/image-orientation/drawImage-from-blob.tentative.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/manual/drawing-images-to-the-canvas/image-orientation/drawImage-from-blob.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html class="reftest-wait">
<head>
<meta charset="utf-8">
<title>createImageBitmap and drawImage from a blob with image orientation: from-image</title>
<link rel="author" title="Stephen Chenney" href="mailto:schenney@chromium.org">
<link rel="help" href="https://drafts.csswg.org/css-images-3/#propdef-image-orientation">
<link rel="match" href="drawImage-from-blob-ref.html">
  <script>
  function makeBlob() {
    return new Promise(function(resolve, reject) {
        var xhr = new XMLHttpRequest();
        xhr.open("GET", '/css/css-images/image-orientation/support/exif-orientation-8-llo.jpg');
        xhr.responseType = 'blob';
        xhr.send();
        xhr.onload = function() {
          resolve(xhr.response);
        };
    });
  }

  window.onload = function() {
    var cfb = document.getElementById("canvasWithFileBitmap");
    makeBlob().then(function(blob){createImageBitmap(blob).then(bitmap => {
        cfb.getContext("2d").drawImage(bitmap, 0, 0, 150, 150 * bitmap.height / bitmap.width);
        window.requestAnimationFrame(() => {
          document.documentElement.removeAttribute("class");
        });
      });
    });
  }
</script>
</head>
<body>
  <canvas id="canvasWithFileBitmap" width="150" height="300"></canvas>
</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/canvas/element/manual/drawing-images-to-the-canvas/image-orientation/drawImage-from-blob.tentative.html"
}
```
