# html/browsers/origin/relaxing-the-same-origin-restriction/sandboxed-document_domain.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/origin/relaxing-the-same-origin-restriction/sandboxed-document_domain.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Sandboxed document.domain</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
  test(() => {
    assert_throws_dom("SecurityError", () => { document.domain = document.domain });
  });
  test(() => {
    assert_throws_dom("SecurityError", () => { (new Document).domain = document.domain });
  });
  test(() => {
    assert_throws_dom("SecurityError", () => { document.implementation.createHTMLDocument().domain = document.domain });
  });
  test(() => {
    assert_throws_dom("SecurityError", () => { document.implementation.createDocument(null, "").domain = document.domain });
  });
  test(() => {
    assert_throws_dom("SecurityError", () => { document.createElement("template").content.ownerDocument.domain = document.domain });
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
  "source_name": "html/browsers/origin/relaxing-the-same-origin-restriction/sandboxed-document_domain.html"
}
```
