# html/cross-origin-embedder-policy/meta-http-equiv.https.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/cross-origin-embedder-policy/meta-http-equiv.https.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta http-equiv="Cross-Origin-Embedder-Policy" content="require-corp"><!-- should not be supported -->
<title>Cross-Origin-Embedder-Policy in &lt;meta http-equiv></title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<div id=log></div>
<script>
async_test(t => {
  const frame = document.createElement("iframe");
  t.add_cleanup(() => frame.remove());
  frame.src = "/common/blank.html";
  document.body.append(frame);
  assert_equals(frame.contentDocument.URL, "about:blank");
  assert_equals(frame.contentDocument.body.localName, "body");
  frame.onload = t.step_func_done(() => {
    assert_equals(frame.contentDocument.URL, `${location.protocol}//${location.host}/common/blank.html`);
    assert_equals(frame.contentDocument.body.localName, "body");
  });
}, `<meta http-equiv="Cross-Origin-Embedder-Policy" content="require-corp"> top-level: navigating a frame to "none" should not fail`);
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
  "source_name": "html/cross-origin-embedder-policy/meta-http-equiv.https.html"
}
```
