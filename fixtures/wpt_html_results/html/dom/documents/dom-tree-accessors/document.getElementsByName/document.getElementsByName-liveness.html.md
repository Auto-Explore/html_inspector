# html/dom/documents/dom-tree-accessors/document.getElementsByName/document.getElementsByName-liveness.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/documents/dom-tree-accessors/document.getElementsByName/document.getElementsByName-liveness.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Document.getElementsByName: liveness</title>
<link rel="author" title="Intel" href="http://www.intel.com">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<script>
test(function() {
  var input = document.createElement("input"),
      embed = document.createElement("embed");
  input.setAttribute("name", "test");
  input.setAttribute("type", "text");
  embed.setAttribute("name", "test");
  document.body.appendChild(input);
  this.add_cleanup(function() { document.body.removeChild(input) });
  var e = document.getElementsByName("test");
  assert_true(e instanceof NodeList);
  assert_equals(e.length, 1);

  document.body.appendChild(embed);
  assert_equals(e.length, 2);

  document.body.removeChild(embed);
  assert_equals(e.length, 1);
}, "Document.getElementsByName() should be a live collection");
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
  "source_name": "html/dom/documents/dom-tree-accessors/document.getElementsByName/document.getElementsByName-liveness.html"
}
```
