# html/cross-origin-embedder-policy/data.https.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/cross-origin-embedder-policy/data.https.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="resources/script-factory.js"></script>
<div id=log></div>
<script>
async_test(t => {
  window.addEventListener("message", t.step_func_done(({ data }) => {
    assert_equals(data.id, "");
    assert_equals(data.origin, "null");
    assert_false(data.sameOriginNoCORPSuccess); // This is effectively a no-op for this test
    assert_true(data.crossOriginNoCORPFailure, "Cross-origin without CORP did not fail");
  }));
  const frame = document.createElement("iframe");
  t.add_cleanup(() => frame.remove());
  frame.src = `data:text/html,<script>${encodeURIComponent(createScript("null", window.origin))}<\/script>`;
  document.body.append(frame);
}, "Cross-Origin-Embedder-Policy and data: URLs");
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
  "source_name": "html/cross-origin-embedder-policy/data.https.html"
}
```
