# html/dom/documents/dom-tree-accessors/document.getElementsByName/document.getElementsByName-interface.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/documents/dom-tree-accessors/document.getElementsByName/document.getElementsByName-interface.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Document.getElementsByName: interfaces</title>
<link rel="author" title="Ms2ger" href="mailto:Ms2ger@gmail.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#dom-document-getelementsbyname">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<script>
test(function() {
  var collection = document.getElementsByName("name");
  assert_class_string(collection, "NodeList");
  assert_true(collection instanceof NodeList, "Should be a NodeList");
  assert_false(collection instanceof HTMLCollection,
               "Should not be a HTMLCollection");
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
  "source_name": "html/dom/documents/dom-tree-accessors/document.getElementsByName/document.getElementsByName-interface.html"
}
```
