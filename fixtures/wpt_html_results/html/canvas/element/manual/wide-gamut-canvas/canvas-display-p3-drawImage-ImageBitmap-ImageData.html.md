# html/canvas/element/manual/wide-gamut-canvas/canvas-display-p3-drawImage-ImageBitmap-ImageData.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/manual/wide-gamut-canvas/canvas-display-p3-drawImage-ImageBitmap-ImageData.html",
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
// Test that ImageBitmaps created from ImageData sources with different color
// spaces can be drawn into sRGB and Display P3 canvases, by reading pixels with
// getImageData() as sRGB and Display P3 values.
function runTest(sourceColorSpace, destinationColorSpace, colors) {
    for (let [sourceColorString, expectedColor] of Object.entries(colors)) {
        for (let cropSource of [false, true]) {
            async_test(function(t) {
                let sourceColor = sourceColorString.split(",").map(x => +x);

                let sourceImageData = new ImageData(4, 4, { colorSpace: sourceColorSpace });
                for (let i = 0; i < 4 * 4 * 4; i += 4) {
                    for (let c = 0; c < 4; ++c)
                        sourceImageData.data[i + c] = sourceColor[c];
                }

                let imageBitmapPromise;
                if (cropSource)
                    imageBitmapPromise = createImageBitmap(sourceImageData, 2, 2, 2, 2);
                else
                    imageBitmapPromise = createImageBitmap(sourceImageData);

                imageBitmapPromise.then(t.step_func_done(function(imageBitmap) {
                    let destination = document.createElement("canvas");
                    destination.width = 2;
                    destination.height = 2;

                    let destinationContext = destination.getContext("2d", { colorSpace: destinationColorSpace });
                    destinationContext.drawImage(imageBitmap, 0, 0);

                    let destinationImageData = destinationContext.getImageData(1, 1, 1, 1);

                    assert_true(pixelsApproximatelyEqual(destinationImageData.data, expectedColor), `Actual pixel value ${[...destinationImageData.data]} is approximately equal to ${expectedColor}.`);
                }));
            }, `Source ${sourceColorSpace}, destination ${destinationColorSpace}, color ${sourceColorString}, cropSource=${cropSource}`);
        }
    }
}

runTest("srgb", "display-p3", fromSRGBToDisplayP3);
runTest("display-p3", "srgb", fromDisplayP3ToSRGB);
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
  "source_name": "html/canvas/element/manual/wide-gamut-canvas/canvas-display-p3-drawImage-ImageBitmap-ImageData.html"
}
```
