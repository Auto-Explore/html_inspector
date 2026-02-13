# html/cross-origin-opener-policy/coop-csp-sandbox-navigate.https.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/cross-origin-opener-policy/coop-csp-sandbox-navigate.https.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>CSP sandbox popup navigate to Cross-Origin-Opener-Policy document should work</title>
<meta name="timeout" content="long">
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
    let popup;
    channel.onmessage = t.step_func_done(e => {
      assert_equals(e.data.name, '', 'e.data.name');
      assert_false(e.data.opener, 'e.data.opener');
      // `popup` is still the WindowProxy that holds the CSP sandbox document, not the
      // after-navigation COOP document. The CSP sandbox only applies to the before navigation
      // document/window.
      assert_true(popup.closed, 'popup.closed');
      // Same-origin check (with the CSP sandbox document) should not throw when 'allow-same-origin'
      if (sandboxValue.includes('allow-same-origin')) {
        assert_true(!!popup.document, 'same-origin check');
      } else {
        assert_throws_dom("SecurityError", () => { popup.document; }, 'same-origin check');
      }
    });
    const navigateTo = `/html/cross-origin-opener-policy/resources/coop-coep.py?coop=same-origin&coep=&channel=${channel.name}`;
    popup = window.open(`resources/csp-sandbox.py?coop=&coep=&sandbox=${sandboxValue}&channel=&navigate=${encodeURIComponent(navigateTo)}`, sandboxValue.replace(/ /g, '_'));
    t.add_cleanup(() => { popup.close(); });
    addEventListener('load', t.step_func(() => {
      t.step_timeout(() => {
        assert_unreached('Navigation from CSP sandbox to COOP document failed')
      }, 10000);
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
  "source_name": "html/cross-origin-opener-policy/coop-csp-sandbox-navigate.https.html"
}
```
