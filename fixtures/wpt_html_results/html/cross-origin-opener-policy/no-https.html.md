# html/cross-origin-opener-policy/no-https.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/cross-origin-opener-policy/no-https.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta name=timeout content=long>
<title>Cross-Origin-Opener-Policy requires secure contexts</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<script>
async_test(t => {
  const popup = window.open("resources/call-functionCalledByOpenee.html");
  t.add_cleanup(() => {
    popup.close();
  });
  window.functionCalledByOpenee = t.step_func_done(() => {
    assert_false(popup.closed);
  });
  assert_equals(window, popup.opener);
}, "Cross-Origin-Opener-Policy only works over secure contexts");

test(() => {
  assert_false(window.crossOriginIsolated);
}, "Bonus: window.crossOriginIsolated");
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
  "source_name": "html/cross-origin-opener-policy/no-https.html"
}
```
