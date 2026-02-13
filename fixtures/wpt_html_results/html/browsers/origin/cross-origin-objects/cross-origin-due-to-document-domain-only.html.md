# html/browsers/origin/cross-origin-objects/cross-origin-due-to-document-domain-only.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/origin/cross-origin-objects/cross-origin-due-to-document-domain-only.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Cross-origin due to document.domain</title>
<meta charset=utf-8>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<div id=log></div>
<iframe src=resources/cross-origin-due-to-document-domain-only-helper.html></iframe>
<script>
async_test(t => {
  onload = t.step_func_done(() => {
    const frame = document.querySelector("iframe");
    const innerSelf = self[0];
    const innerLocation = innerSelf.location;
    const innerDocument = innerSelf.document;
    assert_equals(innerLocation.host, location.host);
    assert_true(innerSelf.expandosForever);
    assert_true(innerLocation.expandosForever);
    assert_equals(frame.contentWindow, innerSelf);
    assert_equals(frame.contentDocument, innerDocument);
    innerSelf.setDocumentDomain();
    assert_throws_dom("SecurityError", () => innerSelf.expandosForever);
    assert_throws_dom("SecurityError", () => innerLocation.expandosForever);
    assert_throws_dom("SecurityError", () => innerLocation.host);
    assert_equals(innerSelf.parent, self);
    assert_throws_dom("SecurityError", () => innerSelf.frameElement);
    assert_throws_dom("SecurityError", () => innerLocation.reload());
    assert_equals(frame.contentWindow, innerSelf);
    assert_equals(frame.contentDocument, null);
    // Cross-origin Document object obtained before it became cross-origin has no protections
    assert_equals(innerDocument.URL, frame.src);
  });
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
  "source_name": "html/browsers/origin/cross-origin-objects/cross-origin-due-to-document-domain-only.html"
}
```
