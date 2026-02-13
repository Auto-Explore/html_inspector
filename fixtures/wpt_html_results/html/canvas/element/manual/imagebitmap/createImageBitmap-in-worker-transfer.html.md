# html/canvas/element/manual/imagebitmap/createImageBitmap-in-worker-transfer.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/manual/imagebitmap/createImageBitmap-in-worker-transfer.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<title>createImageBitmap in worker and transfer</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id=log></div>
<script>
promise_test(function(t) {
    return new Promise(function(resolve, reject) {
        var worker = new Worker("createImageBitmap-worker.js");
        worker.addEventListener("message", function(evt) {
            var bitmap = evt.data;
            assert_equals(bitmap.width, 20);
            assert_equals(bitmap.height, 20);
            resolve();
        });
        worker.postMessage('test');
    });
}, 'Transfer ImageBitmap created in worker');
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
  "source_name": "html/canvas/element/manual/imagebitmap/createImageBitmap-in-worker-transfer.html"
}
```
