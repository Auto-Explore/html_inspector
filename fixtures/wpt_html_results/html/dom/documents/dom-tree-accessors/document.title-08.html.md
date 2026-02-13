# html/dom/documents/dom-tree-accessors/document.title-08.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/documents/dom-tree-accessors/document.title-08.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel="author" title="Denis Ah-Kang" href="mailto:denis@w3.org">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#document.title">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<script>
test(function() {
  assert_equals(document.title, "");
}, "No title element");

test(function() {
  var title = document.createElement("title");
  title.appendChild(document.createTextNode("PASS"));
  document.head.appendChild(title);
  assert_equals(document.title, "PASS");

  title.appendChild(document.createTextNode("PASS2"));
  title.appendChild(document.createTextNode("PASS3"));
  assert_equals(document.title, "PASSPASS2PASS3");
}, "title element contains multiple child text nodes");
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
  "source_name": "html/dom/documents/dom-tree-accessors/document.title-08.html"
}
```
