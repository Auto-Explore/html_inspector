# html/dom/documents/dom-tree-accessors/document.links.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/documents/dom-tree-accessors/document.links.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<title>Document.links</title>
<link rel="author" title="Intel" href="http://www.intel.com">
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<div id=log></div>
<div id=test>
<a href=""></a>
<a href=""></a>
</div>
<script>
test(function() {
  var div = document.getElementById("test");
  var links = document.links;
  assert_true(links instanceof HTMLCollection);
  assert_equals(links.length, 2);

  var a = document.createElement("a");
  a.setAttribute("href", "");
  div.appendChild(a);
  assert_equals(links.length, 3);

  div.removeChild(a);
  assert_equals(links.length, 2);
}, "Document.links should be a live collection");
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
  "source_name": "html/dom/documents/dom-tree-accessors/document.links.html"
}
```
