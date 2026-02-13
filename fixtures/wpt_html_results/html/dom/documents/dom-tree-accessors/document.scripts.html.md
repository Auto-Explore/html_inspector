# html/dom/documents/dom-tree-accessors/document.scripts.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/documents/dom-tree-accessors/document.scripts.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<title>Document.scripts</title>
<link rel="author" title="Intel" href="http://www.intel.com">
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<div id=log></div>
<script>
test(function() {
  var scripts = document.scripts;
  assert_true(scripts instanceof HTMLCollection);
  assert_equals(scripts.length, 3);

  var script = document.createElement("script");
  document.body.appendChild(script);
  assert_equals(scripts.length, 4);

  document.body.removeChild(script);
  assert_equals(scripts.length, 3);
}, "Document.scripts should be a live collection");
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
  "source_name": "html/dom/documents/dom-tree-accessors/document.scripts.html"
}
```
