# html/canvas/element/manual/imagebitmap/createImageBitmap-origin.sub.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/manual/imagebitmap/createImageBitmap-origin.sub.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<title>createImageBitmap: origin-clean flag</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/common/get-host-info.sub.js"></script>
<script src="/common/media.js"></script>
<script src="/html/canvas/resources/canvas-tests.js"></script>
<div id=log></div>
<script>

function assert_origin_unclean_getImageData(bitmap) {
  const context = document.createElement("canvas").getContext("2d");
  context.drawImage(bitmap, 0, 0);
  assert_throws_dom("SecurityError", () => {
    context.getImageData(0, 0, 1, 1);
  });
}

function assert_origin_unclean_drawImage(bitmap) {
  const canvas = document.createElement('canvas');
  const ctx = canvas.getContext('2d');
  ctx.drawImage(bitmap, 0, 0);
  assert_throws_dom('SecurityError', () => canvas.toDataURL());
}

function assert_origin_unclean_transferFromImageBitmap(bitmap) {
  var canvas = document.createElement('canvas');
  var ctx = canvas.getContext('bitmaprenderer');
  ctx.transferFromImageBitmap(bitmap);
  assert_throws_dom('SecurityError', () => canvas.toDataURL());
}

forEachCanvasSource(get_host_info().HTTP_REMOTE_ORIGIN,
                    get_host_info().HTTP_ORIGIN,
                    (name, factory) => {
  promise_test(function() {
    return factory().then(createImageBitmap).then(assert_origin_unclean_getImageData);
  }, `${name}: origin unclear getImageData`);
  promise_test(function() {
    return factory().then(createImageBitmap).then(assert_origin_unclean_drawImage);
  }, `${name}: origin unclear 2dContext.drawImage`);
  promise_test(function() {
    return factory().then(createImageBitmap).then(assert_origin_unclean_transferFromImageBitmap);
  }, `${name}: origin unclear bitmaprenderer.transferFromImageBitmap`);
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
  "source_name": "html/canvas/element/manual/imagebitmap/createImageBitmap-origin.sub.html"
}
```
