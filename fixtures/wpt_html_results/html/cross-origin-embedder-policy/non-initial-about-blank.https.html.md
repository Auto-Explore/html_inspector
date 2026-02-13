# html/cross-origin-embedder-policy/non-initial-about-blank.https.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/cross-origin-embedder-policy/non-initial-about-blank.https.html",
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
<div id=log></div>
<script>
async_test(t => {
  const frame = document.createElement("iframe");
  t.add_cleanup(() => frame.remove());
  let i = 0;
  frame.onload = t.step_func(() => {
    i++;
    assert_equals(frame.contentDocument.URL, "about:blank");
    frame.src = "about:blank";
    if (i == 2) {
      t.done();
    }
  });
  document.body.append(frame);
}, "Cross-Origin-Embedder-Policy and about:blank");
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
  "source_name": "html/cross-origin-embedder-policy/non-initial-about-blank.https.html"
}
```
