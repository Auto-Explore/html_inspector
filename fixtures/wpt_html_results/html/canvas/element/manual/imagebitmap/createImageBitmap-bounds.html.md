# html/canvas/element/manual/imagebitmap/createImageBitmap-bounds.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/manual/imagebitmap/createImageBitmap-bounds.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<title>createImageBitmap: clipping to the bitmap</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/html/canvas/resources/canvas-tests.js"></script>
<div id="results"></div>
<script>
const color = 204;
function testClip( name, sx, sy, sw, sh, expectedColors, expectedWidth, expectedHeight ) {
    promise_test(function(t) {
        return new Promise(function(resolve, reject) {
            const image = new Image();
            image.onload = function() { resolve(image); };
            image.onerror = function() { reject(); };
            image.src = "/images/green-16x16.png";
        }).then(function(image) {
            return createImageBitmap(image, sx, sy, sw, sh);
        }).then(function(imageBitmap) {

            assert_equals(imageBitmap.width, expectedWidth);
            assert_equals(imageBitmap.height, expectedHeight);

            const canvas = document.createElement("canvas");
            canvas.width = 16;
            canvas.height = 16;

            // debug
            document.getElementById("results").append(canvas);
            canvas.setAttribute("style", "width: 100px; height: 100px;");

            const ctx = canvas.getContext("2d");
            ctx.fillStyle = `rgb(${color}, ${color}, ${color})`;
            ctx.fillRect(0, 0, 20, 20);
            ctx.drawImage(imageBitmap, 0, 0);

            for (let [x, y, r, g, b, a] of expectedColors) {
                _assertPixel(canvas, x,y, r,g,b,a);
            }
        });
    }, name);
}
testClip( "simple clip inside",
    8, 8, 8, 8, [
        [ 4,  4, 0,255,0,255],           [12,  4, color,color,color,255],
        [ 4, 12, color,color,color,255], [12, 12, color,color,color,255]
    ], 8, 8
);
testClip( "clip outside negative",
    -8, -8, 16, 16, [
        [ 4,  4, color,color,color,255], [12, 4,  color,color,color,255],
        [ 4, 12, color,color,color,255], [12, 12, 0,255,0,255]
    ], 16, 16
);
testClip( "clip outside positive",
    8, 8, 16, 16, [
        [ 4,  4, 0,255,0,255],           [12,  4, color,color,color,255],
        [ 4, 12, color,color,color,255], [12, 12, color,color,color,255]
    ], 16, 16
);
testClip( "clip inside using negative width and height",
    24, 24, -16, -16, [
        [ 4,  4, 0,255,0,255],           [12,  4, color,color,color,255],
        [ 4, 12, color,color,color,255], [12, 12, color,color,color,255]
    ], 16, 16
);
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
  "source_name": "html/canvas/element/manual/imagebitmap/createImageBitmap-bounds.html"
}
```
