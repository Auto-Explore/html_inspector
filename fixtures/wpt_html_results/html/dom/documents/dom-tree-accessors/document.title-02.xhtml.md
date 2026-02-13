# html/dom/documents/dom-tree-accessors/document.title-02.xhtml

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/documents/dom-tree-accessors/document.title-02.xhtml",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<html xmlns="http://www.w3.org/1999/xhtml">
<head>
<title>document.title with head blown away</title>
<link rel="author" title="Ms2ger" href="mailto:ms2ger@gmail.com"/>
<link rel="help" href="https://html.spec.whatwg.org/multipage/#document.title"/>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
</head>
<body>
<div id="log"></div>
<script>
test(function() {
  assert_equals(document.title, "document.title with head blown away");
})
test(function() {
  var head = document.getElementsByTagName("head")[0];
  assert_true(!!head, "Head gone?!")
  head.parentNode.removeChild(head);
  assert_false(!!document.getElementsByTagName("head")[0], "Head still there?!")
  document.title = "FAIL";
  assert_equals(document.title, "");
})
test(function() {
  var title2 = document.createElement("title");
  title2.appendChild(document.createTextNode("PASS"));
  document.body.appendChild(title2);
  assert_equals(document.title, "PASS");
})
test(function() {
  var title3 = document.createElement("title");
  title3.appendChild(document.createTextNode("PASS2"));
  document.documentElement.insertBefore(title3, document.body);
  assert_equals(document.title, "PASS2");
})
</script>
</body>
</html>
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
  "source_name": "html/dom/documents/dom-tree-accessors/document.title-02.xhtml"
}
```
