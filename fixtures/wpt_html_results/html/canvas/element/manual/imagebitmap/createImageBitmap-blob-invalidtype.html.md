# html/canvas/element/manual/imagebitmap/createImageBitmap-blob-invalidtype.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/manual/imagebitmap/createImageBitmap-blob-invalidtype.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<title>createImageBitmap: blob with wrong mime type</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/html/canvas/resources/canvas-tests.js"></script>
<script>
promise_test(t => {
  // Source: https://commons.wikimedia.org/wiki/File:1x1.png (Public Domain)
  const IMAGE = atob("iVBORw0KGgoAAAANSUhEUgAAAAEAAAABAQMAAAAl21bKAAAAA1BMVEUAA" +
                     "ACnej3aAAAAAXRSTlMAQObYZgAAAApJREFUCNdjYAAAAAIAAeIhvDMAAAAASUVORK5CYII=");

  let bytes = new Array(IMAGE.length);
  for (let i = 0; i < IMAGE.length; i++) {
    bytes[i] = IMAGE.charCodeAt(i);
  }

  let blob = new Blob([new Uint8Array(bytes)], { type: "text/html"});

  return window.createImageBitmap(blob)
    .then(imageBitmap => {
      assert_true(true, "Image created!");
      assert_equals(imageBitmap.width, 1, "Image is 1x1");
      assert_equals(imageBitmap.height, 1, "Image is 1x1");
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
  "source_name": "html/canvas/element/manual/imagebitmap/createImageBitmap-blob-invalidtype.html"
}
```
