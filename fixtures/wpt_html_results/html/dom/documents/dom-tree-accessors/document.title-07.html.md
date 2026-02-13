# html/dom/documents/dom-tree-accessors/document.title-07.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/documents/dom-tree-accessors/document.title-07.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Document.title and DOMImplementation.createHTMLDocument</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/dom/nodes/DOMImplementation-createHTMLDocument.js"></script>
<div id="log"></div>
<script>
createHTMLDocuments(function(doc, expectedtitle, normalizedtitle) {
  assert_equals(doc.title, normalizedtitle)
})
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
  "source_name": "html/dom/documents/dom-tree-accessors/document.title-07.html"
}
```
