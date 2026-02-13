# html/webappapis/dynamic-markup-insertion/document-writeln/document.writeln-02.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/dynamic-markup-insertion/document-writeln/document.writeln-02.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>document.writeln and null/undefined</title>
<link rel="author" title="Ms2ger" href="mailto:ms2ger@gmail.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#dom-document-writeln%28%29">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#documents-in-the-dom">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<script>
test(function() {
  var iframe = document.createElement("iframe");
  document.body.appendChild(iframe);
  doc = iframe.contentDocument;
  test(function() {
    doc.open();
    doc.writeln(null);
    doc.close();
    assert_equals(doc.documentElement.textContent, "null\n");
  }, "document.writeln(null)");
  test(function() {
    doc.open();
    doc.writeln(undefined);
    doc.close();
    assert_equals(doc.documentElement.textContent, "undefined\n");
  }, "document.writeln(undefined)");
}, "Calling document.writeln with null and undefined");
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
  "source_name": "html/webappapis/dynamic-markup-insertion/document-writeln/document.writeln-02.html"
}
```
