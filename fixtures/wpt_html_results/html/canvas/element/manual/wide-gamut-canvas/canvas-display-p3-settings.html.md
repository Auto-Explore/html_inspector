# html/canvas/element/manual/wide-gamut-canvas/canvas-display-p3-settings.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/manual/wide-gamut-canvas/canvas-display-p3-settings.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
// Test that creating canvas contexts and ImageData objects respect the
// requested color space.

for (let contextColorSpace of [undefined, "srgb", "display-p3"]) {
    for (let imageDataColorSpace of [undefined, "srgb", "display-p3"]) {
        test(function() {
            let contextSettings = { };
            if (contextColorSpace)
                contextSettings.colorSpace = contextColorSpace;
            let resolvedContextColorSpace = contextColorSpace || "srgb";

            let canvas = document.createElement("canvas");
            let ctx = canvas.getContext("2d", contextSettings);
            assert_equals(ctx.getContextAttributes().colorSpace, resolvedContextColorSpace, `CanvasRenderingContext2DSettings.colorSpace when set to ${contextColorSpace}`);

            let imageDataSettings = { };
            if (imageDataColorSpace)
                imageDataSettings.colorSpace = imageDataColorSpace;
            let resolvedImageDataColorSpace = imageDataColorSpace || resolvedContextColorSpace;

            let imageData = ctx.getImageData(0, 0, 1, 1, imageDataSettings);
            assert_equals(imageData.colorSpace, resolvedImageDataColorSpace, `getImageData() colorSpace when set to ${imageDataColorSpace}`);

            imageData = ctx.createImageData(1, 1, imageDataSettings);
            assert_equals(imageData.colorSpace, resolvedImageDataColorSpace, `createImageData() colorSpace when set to ${imageDataColorSpace}`);

            imageData = ctx.createImageData(imageData);
            assert_equals(imageData.colorSpace, resolvedImageDataColorSpace, `Cloned ImageData colorSpace when set to ${imageDataColorSpace}`);
        }, `Context ${contextColorSpace}, ImageData ${imageDataColorSpace}`);
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
  "source_name": "html/canvas/element/manual/wide-gamut-canvas/canvas-display-p3-settings.html"
}
```
