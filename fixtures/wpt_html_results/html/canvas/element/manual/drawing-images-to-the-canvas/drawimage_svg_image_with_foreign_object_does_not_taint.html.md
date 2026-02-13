# html/canvas/element/manual/drawing-images-to-the-canvas/drawimage_svg_image_with_foreign_object_does_not_taint.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/manual/drawing-images-to-the-canvas/drawimage_svg_image_with_foreign_object_does_not_taint.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Draw an SVG image with a foreignObject to a canvas</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
function loadImage(url) {
  return new Promise(resolve => {
    const image = new window.Image();
    image.onload = () => {
      resolve(image);
    };
    image.src = url;
  });
}

promise_test(async (t) => {
  // Load a data URL for an SVG image with a foreign object.
  const url = 'data:image/svg+xml,<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100"><foreignObject></foreignObject></svg>';
  const image = await loadImage(url);

  // Draw the image to a canvas.
  const canvas = document.createElement('canvas');
  const context = canvas.getContext('2d');
  canvas.width = image.width;
  canvas.height = image.height;
  context.drawImage(image, 0, 0);

  // The canvas should not be tainted, so the following shouldn't throw.
  assert_true(canvas.toDataURL().length > 0);
}, 'Canvas should not be tainted after drawing SVG including <foreignObject>');
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
  "source_name": "html/canvas/element/manual/drawing-images-to-the-canvas/drawimage_svg_image_with_foreign_object_does_not_taint.html"
}
```
