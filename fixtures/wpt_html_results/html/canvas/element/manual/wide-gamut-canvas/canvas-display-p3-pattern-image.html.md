# html/canvas/element/manual/wide-gamut-canvas/canvas-display-p3-pattern-image.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/manual/wide-gamut-canvas/canvas-display-p3-pattern-image.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="canvas-display-p3.js"></script>
<script>
// Test that patterns created from images with different bit depths and color
// profiles into can be drawn into sRGB and Display P3 canvases, by reading
// pixels with getImageData() as sRGB and Display P3 values.
for (let [filename, expectedPixels] of Object.entries(imageTests)) {
    for (let contextColorSpace of ["srgb", "display-p3"]) {
        for (let imageDataColorSpace of ["srgb", "display-p3"]) {
            async_test(function(t) {
                let image = new Image();
                image.onload = t.step_func_done(function() {

                    let canvas = document.createElement("canvas");
                    canvas.width = 4;
                    canvas.height = 4;

                    let ctx = canvas.getContext("2d", { colorSpace: contextColorSpace });
                    ctx.fillStyle = ctx.createPattern(image, "repeat");
                    ctx.fillRect(0, 0, 4, 4);

                    let imageData = ctx.getImageData(2, 2, 1, 1, { colorSpace: imageDataColorSpace });

                    let expected = expectedPixels[`${contextColorSpace} ${imageDataColorSpace}`];
                    assert_true(pixelsApproximatelyEqual(imageData.data, expected), `Actual pixel value ${[...imageData.data]} is approximately equal to ${expected}.`);

                    t.done();

                });
                image.src = `resources/${filename}`;
            }, `${filename}, Context ${contextColorSpace}, ImageData ${imageDataColorSpace}`);
        }
    }
}
</script>
```

```json
{
  "messages": [
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
  "source_name": "html/canvas/element/manual/wide-gamut-canvas/canvas-display-p3-pattern-image.html"
}
```
