# html/canvas/element/manual/wide-gamut-canvas/canvas-display-p3-drawImage-ImageBitmap-cloned.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/manual/wide-gamut-canvas/canvas-display-p3-drawImage-ImageBitmap-cloned.html",
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
// Test that drawing structured cloned ImageBitmaps with different image source
// bit depths and color profiles into sRGB and Display P3 canvases works, by
// reading pixels with getImageData() as sRGB and Display P3 values.

let nextTestID = 0;

class Test {
    constructor(testConfiguration) {
        Object.assign(this, testConfiguration);
        this.testID = nextTestID++;
    }

    run() {
        let self = this;
        async_test(function(t) {
            self.t = t;
            self.image = new Image();
            self.image.onload = t.step_func(self.onImageLoaded.bind(self));
            self.image.src = `resources/${self.filename}`;
        }, `${this.filename}, Context ${this.contextColorSpace}, ImageData ${this.imageDataColorSpace}, cropSource=${this.cropSource}`);
    }

    onImageLoaded() {
        let imageBitmapPromise;
        if (this.cropSource)
            imageBitmapPromise = createImageBitmap(this.image, 1, 1, 1, 1);
        else
            imageBitmapPromise = createImageBitmap(this.image);
        imageBitmapPromise.then(this.t.step_func(this.onImageBitmapCreated.bind(this)));
    }

    onImageBitmapCreated(imageBitmap) {
        window.addEventListener("message", this.t.step_func(this.onMessage.bind(this)));
        window.postMessage({ imageBitmap, testID: this.testID });
    }

    onMessage(message) {
        if (message.data.testID != this.testID)
            return;

        let canvas = document.createElement("canvas");
        canvas.width = 2;
        canvas.height = 2;

        let ctx = canvas.getContext("2d", { colorSpace: this.contextColorSpace });
        ctx.drawImage(message.data.imageBitmap, 0, 0);

        let imageData = ctx.getImageData(0, 0, 1, 1, { colorSpace: this.imageDataColorSpace });

        let expected = this.expectedPixels[`${this.contextColorSpace} ${this.imageDataColorSpace}`];
        assert_true(pixelsApproximatelyEqual(imageData.data, expected), `Actual pixel value ${[...imageData.data]} is approximately equal to ${expected}.`);

        this.t.done();
    }
}

for (let [filename, expectedPixels] of Object.entries(imageTests)) {
    for (let contextColorSpace of ["srgb", "display-p3"]) {
        for (let imageDataColorSpace of ["srgb", "display-p3"]) {
            for (let cropSource of [false, true])
                new Test({ filename, expectedPixels, contextColorSpace, imageDataColorSpace, cropSource }).run();
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
  "source_name": "html/canvas/element/manual/wide-gamut-canvas/canvas-display-p3-drawImage-ImageBitmap-cloned.html"
}
```
