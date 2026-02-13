# html/canvas/element/manual/wide-gamut-canvas/canvas-display-p3-drawImage-video.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/manual/wide-gamut-canvas/canvas-display-p3-drawImage-video.html",
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
<body>
<script>
// Test that drawing videos with different color spaces into sRGB and Display P3
// canvases works, by reading pixels with getImageData() as sRGB and Display P3
// values.
for (let [filenameBase, expectedPixels] of Object.entries(videoTests)) {
    for (let contextColorSpace of ["srgb", "display-p3"]) {
        for (let imageDataColorSpace of ["srgb", "display-p3"]) {
            for (let scaleImage of [false, true]) {
                promise_test(async function(t) {

                    let video = document.createElement("video");
                    for (let format of ["mp4", "webm"]) {
                        let source = document.createElement("source");
                        source.src = `resources/${filenameBase}.${format}`;
                        source.type = `video/${format}`;
                        video.append(source);
                    }

                    let loadedData = new Promise(resolver => video.onloadeddata = resolver);

                    document.body.append(video);
                    await video.play();
                    await loadedData;
                    await new Promise(requestAnimationFrame);

                    let canvas = document.createElement("canvas");
                    canvas.width = 2;
                    canvas.height = 2;

                    let ctx = canvas.getContext("2d", { colorSpace: contextColorSpace });
                    if (scaleImage)
                        ctx.drawImage(video, 0, 0, 10, 10);
                    else
                        ctx.drawImage(video, 0, 0);
                    video.remove();

                    let imageData = ctx.getImageData(0, 0, 1, 1, { colorSpace: imageDataColorSpace });

                    let expected = expectedPixels[`${contextColorSpace} ${imageDataColorSpace}`];
                    assert_true(pixelsApproximatelyEqual(imageData.data, expected), `Actual pixel value ${[...imageData.data]} is approximately equal to ${expected}.`);

                }, `${filenameBase}, Context ${contextColorSpace}, ImageData ${imageDataColorSpace}, scaleImage=${scaleImage}`);
            }
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
  "source_name": "html/canvas/element/manual/wide-gamut-canvas/canvas-display-p3-drawImage-video.html"
}
```
