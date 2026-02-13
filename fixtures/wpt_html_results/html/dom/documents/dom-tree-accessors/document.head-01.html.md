# html/dom/documents/dom-tree-accessors/document.head-01.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/documents/dom-tree-accessors/document.head-01.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>document.head</title>
<link rel="author" title="Ms2ger" href="mailto:ms2ger@gmail.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#dom-document-head">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<script>
test(function() {
  var head = document.getElementsByTagName("head")[0];
  assert_equals(document.head, head);
  document.head = "";
  assert_equals(document.head, head);
  document.head = document.createElement("head");
  assert_equals(document.head, head);
  document.documentElement.appendChild(document.createElement("head"));
  assert_equals(document.head, head);
  var head2 = document.createElement("head");
  document.documentElement.insertBefore(head2, head);
  assert_equals(document.head, head2);
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
  "source_name": "html/dom/documents/dom-tree-accessors/document.head-01.html"
}
```
