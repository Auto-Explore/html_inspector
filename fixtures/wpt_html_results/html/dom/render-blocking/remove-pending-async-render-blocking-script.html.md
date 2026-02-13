# html/dom/render-blocking/remove-pending-async-render-blocking-script.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/render-blocking/remove-pending-async-render-blocking-script.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Removed render-blocking script should not indefinitely block rendering</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script id="target" async blocking="render"
        src="support/dummy-1.js?pipe=trickle(d1)"></script>
<script>
promise_test(async () => {
  const target = document.getElementById('target');
  const newDoc = document.implementation.createHTMLDocument('new document');
  newDoc.documentElement.appendChild(target);

  await new Promise(resolve => requestAnimationFrame(resolve));

  // reqeustAnimationFrame() should be eventually run, but the script removed
  // while pending should not be run.
  assert_equals(window.dummy, undefined);
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
  "source_name": "html/dom/render-blocking/remove-pending-async-render-blocking-script.html"
}
```
