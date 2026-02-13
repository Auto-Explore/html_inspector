# html/infrastructure/safe-passing-of-structured-data/cross-origin-transfer-resizable-arraybuffer.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/infrastructure/safe-passing-of-structured-data/cross-origin-transfer-resizable-arraybuffer.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>postMessage transfer ArrayBuffer cross origin iframe</title>
<script src='/resources/testharness.js'></script>
<script src='/resources/testharnessreport.js'></script>
<script src='/common/get-host-info.sub.js'></script>

<script>

async_test(t => {
  const oopif = document.createElement('iframe');

  window.addEventListener('message', t.step_func((e) => {
    if (e.data === 'started') {
      const rab = new ArrayBuffer(32, { maxByteLength: 1024 });
      oopif.contentWindow.postMessage(rab, '*', [rab]);
    } else {
      assert_equals(e.data, 'byteLength=32,maxByteLength=1024,resizable=true');
      t.done();
    }
  }));

  window.addEventListener('load', () => {
    oopif.src = `${get_host_info().HTTP_REMOTE_ORIGIN}/html/infrastructure/safe-passing-of-structured-data/resources/iframe-resizable-arraybuffer-helper.html`;
    document.body.appendChild(oopif);
  });
}, 'postMessaging resizable ArrayBuffer to OOPIF');

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
  "source_name": "html/infrastructure/safe-passing-of-structured-data/cross-origin-transfer-resizable-arraybuffer.html"
}
```
