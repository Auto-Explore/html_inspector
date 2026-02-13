# html/semantics/tabular-data/the-table-element/insertRow-method-01.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/tabular-data/the-table-element/insertRow-method-01.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>insertRow(): INDEX_SIZE_ERR</title>
<link rel="author" title="Ms2ger" href="mailto:ms2ger@gmail.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#dom-table-insertrow">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<div id="test">
<table>
<tr>
<td>
</table>
</div>
<script>
test(function() {
  var table = document.getElementById("test").getElementsByTagName("table")[0];
  assert_throws_dom("INDEX_SIZE_ERR", function() {
    table.insertRow(-2);
  })
  assert_throws_dom("INDEX_SIZE_ERR", function() {
    table.insertRow(2);
  })
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
  "source_name": "html/semantics/tabular-data/the-table-element/insertRow-method-01.html"
}
```
