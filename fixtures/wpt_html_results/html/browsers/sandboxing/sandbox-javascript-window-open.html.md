# html/browsers/sandboxing/sandbox-javascript-window-open.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/sandboxing/sandbox-javascript-window-open.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<title>window.open in sandbox iframe</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/common/utils.js"></script>
<body>
<script>
promise_test(async test => {
  let message = new Promise(resolve => {
    window.addEventListener("message", event => resolve(event.data));
  });
  let iframe = document.createElement("iframe");
  iframe.sandbox = "allow-scripts allow-popups allow-same-origin";
  iframe.src = "./resources/sandbox-javascript-window-open.html";
  document.body.appendChild(iframe);
  assert_equals(await message, "disallow-document-domain");
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
  "source_name": "html/browsers/sandboxing/sandbox-javascript-window-open.html"
}
```
