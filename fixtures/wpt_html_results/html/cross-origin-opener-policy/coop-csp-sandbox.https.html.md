# html/cross-origin-opener-policy/coop-csp-sandbox.https.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/cross-origin-opener-policy/coop-csp-sandbox.https.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>CSP sandboxed Cross-Origin-Opener-Policy popup should result in a network error</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<script src="/common/utils.js"></script> <!-- Use token() to allow running tests in parallel -->
<div id=log>
<script>
[
  "allow-popups allow-scripts allow-same-origin",
  "allow-popups allow-scripts",
].forEach(sandboxValue => {
  async_test(t => {
    const channel = new BroadcastChannel(token());
    channel.onmessage = t.unreached_func("A COOP popup was created from a CSP-sandboxed popup");
    const popup = window.open(`resources/csp-sandbox.py?coop=same-origin&coep=&sandbox=${sandboxValue}&channel=${channel.name}`);
    t.add_cleanup(() => { popup.close(); });
    addEventListener('load', t.step_func(() => {
      t.step_timeout(() => {
        t.done()
      }, 1500);
    }));
  }, `CSP: sandbox ${sandboxValue}; ${document.title}`);
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
  "source_name": "html/cross-origin-opener-policy/coop-csp-sandbox.https.html"
}
```
