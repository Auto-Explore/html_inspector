# html/canvas/element/manual/imagebitmap/createImageBitmap-drawImage-closed.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/manual/imagebitmap/createImageBitmap-drawImage-closed.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<title>attempt to draw a closed ImageBitmap to a 2d canvas throws INVALID_STATE_ERR</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/html/canvas/resources/canvas-tests.js"></script>
<script>
promise_test(function(t) {
    return new Promise(function(resolve, reject) {
        const image = new Image();
        image.onload = function() { resolve(image); };
        image.onerror = function() { reject(); };
        image.src = "/images/green-16x16.png";
    }).then(function(image) {
        return createImageBitmap(image, 0, 0, 16, 16);
    }).then(function(imageBitmap) {
        imageBitmap.close();

        const canvas = document.createElement("canvas");
        canvas.width = 16;
        canvas.height = 16;

        const ctx = canvas.getContext("2d");
        assert_throws_dom("InvalidStateError", function() { ctx.drawImage(imageBitmap, 0, 0); });
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
  "source_name": "html/canvas/element/manual/imagebitmap/createImageBitmap-drawImage-closed.html"
}
```
