# html/dom/documents/dom-tree-accessors/document.title-not-in-html-svg.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/documents/dom-tree-accessors/document.title-not-in-html-svg.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel="author" title="Rob Buis" href="mailto:rbuis@igalia.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#document.title">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<script>

function newXMLDocument() {
  return document.implementation.createDocument(null, "foo", null);
}

test(function() {
  var doc = newXMLDocument();
  assert_equals(doc.title, "");
  doc.title = "fail";
  assert_equals(doc.title, "");
}, "Should not be able to set document title in XML document");

test(function() {
  var doc = newXMLDocument();
  doc.documentElement.appendChild(document.createElementNS("http://www.w3.org/1999/xhtml", "html:title"));
  assert_equals(doc.title, "");
  doc.title = "fail";
  assert_equals(doc.title, "");
}, "Should not be able to set document title in XML document with html:title element");
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
  "source_name": "html/dom/documents/dom-tree-accessors/document.title-not-in-html-svg.html"
}
```
