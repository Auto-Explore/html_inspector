# html/cross-origin-opener-policy/popup-same-origin-non-initial-about-blank.https.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/cross-origin-opener-policy/popup-same-origin-non-initial-about-blank.https.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Cross-Origin-Opener-Policy: about:blank</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<script>
async_test(t => {
  const popup = window.open("resources/coop-coep.py?coop=same-origin&coep=&navigate=about:blank");
  t.add_cleanup(() => popup.close());
  assert_equals(window, popup.opener);

  popup.onload = t.step_func(() => {
    assert_true(popup.location.href.endsWith("&navigate=about:blank"));
    // Use wait_for_callback as about:blank cannot message back.
    t.step_wait_func_done(() => popup.location.href === "about:blank");
  });
}, "Navigating a popup to about:blank");
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
  "source_name": "html/cross-origin-opener-policy/popup-same-origin-non-initial-about-blank.https.html"
}
```
