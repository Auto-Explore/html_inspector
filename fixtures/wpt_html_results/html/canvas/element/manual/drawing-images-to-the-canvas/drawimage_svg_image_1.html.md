# html/canvas/element/manual/drawing-images-to-the-canvas/drawimage_svg_image_1.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/manual/drawing-images-to-the-canvas/drawimage_svg_image_1.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<title>Load a 100x100 image to a SVG image and draw it to a 100x100 canvas.</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/html/canvas/resources/canvas-tests.js"></script>
<div id="log"></div>
<canvas id="dest" height="100" width="100"></canvas>
<script>
async_test(t => {
  var sourceImg = document.createElementNS('http://www.w3.org/2000/svg', 'image');
  sourceImg.setAttributeNS('http://www.w3.org/1999/xlink', 'href', '/html/canvas/resources/2x2.png');
  sourceImg.width = 100;
  sourceImg.height = 100;

  window.onload = t.step_func_done(() => {
    var destCanvas = document.getElementById('dest');
    var destCtx = destCanvas.getContext('2d');
    destCtx.fillStyle = "#FF0000";
    destCtx.fillRect(0, 0,  destCanvas.width, destCanvas.height);
    destCtx.imageSmoothingEnabled = false;
    // 2 arguments, the dest origin is 0,0
    // The source canvas will copied to the 0,0 position of the destination canvas
    destCtx.drawImage(sourceImg, 0, 0);
    _assertPixel(destCanvas, 0, 0, 253, 140, 245, 255);
    _assertPixel(destCanvas, 0, 99, 253, 140, 245, 255);
    _assertPixel(destCanvas, 99, 0, 253, 140, 245, 255);
    _assertPixel(destCanvas, 99, 99, 253, 140, 245, 255);
  });
});
</script>
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
  "source_name": "html/canvas/element/manual/drawing-images-to-the-canvas/drawimage_svg_image_1.html"
}
```
