# html/canvas/element/manual/imagebitmap/createImageBitmap-transfer.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/manual/imagebitmap/createImageBitmap-transfer.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<title>createImageBitmap transferring test</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/common/media.js"></script>
<script src="common.sub.js"></script>
<script src="/common/get-host-info.sub.js"></script>
<div id=log></div>
<script>
let worker, continuations = {};
setup(function() {
    worker = new Worker("transfer-worker.js");
    worker.addEventListener("message", function(event) {
        let { name, bitmap } = event.data;
        if (continuations.hasOwnProperty(name)) {
            continuations[name](bitmap);
        }
    });
});

for (let { name, factory } of imageSourceTypes) {
    promise_test(function(t) {
        return factory().then(createImageBitmap).then(function(bitmap) {
            assert_equals(bitmap.width, 20);
            assert_equals(bitmap.height, 20);

            worker.postMessage({ name: t.name, bitmap: bitmap }, [bitmap]);

            assert_equals(bitmap.width, 0);
            assert_equals(bitmap.height, 0);

            return new Promise(function(resolve) {
                continuations[t.name] = resolve;
            });
        }).then(function(bitmap) {
            assert_class_string(bitmap, "ImageBitmap");
            assert_equals(bitmap.width, 20);
            assert_equals(bitmap.height, 20);
        });
    }, `Transfer ImageBitmap created from ${name}`);
}

promise_test(async (t) => {
  const url = get_host_info().REMOTE_ORIGIN + '/images/pattern.png';
  const image = await makeMakeHTMLImage(url)();
  const bitmap = await createImageBitmap(image);

  assert_throws_dom('DataCloneError',
                    () => worker.postMessage(bitmap, [bitmap]));
}, 'Transferring a non-origin-clean ImageBitmap throws.');

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
  "source_name": "html/canvas/element/manual/imagebitmap/createImageBitmap-transfer.html"
}
```
