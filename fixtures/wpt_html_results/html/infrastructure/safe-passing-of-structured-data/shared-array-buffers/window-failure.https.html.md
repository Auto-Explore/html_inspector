# html/infrastructure/safe-passing-of-structured-data/shared-array-buffers/window-failure.https.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/infrastructure/safe-passing-of-structured-data/shared-array-buffers/window-failure.https.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/common/get-host-info.sub.js"></script>
<div id=log></div>
<script>
async_test(t => {
  const frame = document.createElement("iframe");
  t.add_cleanup(() => frame.remove());
  frame.src = get_host_info().HTTPS_NOTSAMESITE_ORIGIN + new URL("resources/iframe-failure.html", location).pathname;
  window.onmessage = t.unreached_func("Got a message event, expected a messageerror event");
  window.onmessageerror = t.step_func_done();
  document.body.append(frame);
}, "SharedArrayBuffer and a cross-site <iframe>");
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
  "source_name": "html/infrastructure/safe-passing-of-structured-data/shared-array-buffers/window-failure.https.html"
}
```
