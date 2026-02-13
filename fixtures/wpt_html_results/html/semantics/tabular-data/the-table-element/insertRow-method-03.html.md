# html/semantics/tabular-data/the-table-element/insertRow-method-03.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/tabular-data/the-table-element/insertRow-method-03.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>insertRow(): non-empty table</title>
<link rel="author" title="g-k" href="mailto:greg.guthe@gmail.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#dom-table-insertrow">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<div id="test">
    <table>
        <tbody><tr id="first"></tr><tr id="second"></tr></tbody>
    </table>
</div>
<script>
var HTML = "http://www.w3.org/1999/xhtml";
test(function() {
  var table = document.getElementById("test").getElementsByTagName("table")[0];
  test(function() {
    assert_equals(table.childNodes.length, 3);
    assert_equals(table.rows.length, 2);
  }, "table should start out with two rows")

  var tr;
  test(function() {
    tr = table.insertRow(1);
    assert_equals(tr.localName, "tr");
    assert_equals(tr.namespaceURI, HTML);
    assert_equals(table.getElementsByTagName("tr")[0].id, "first");
    assert_equals(table.getElementsByTagName("tr")[1].id, "");
    assert_equals(table.getElementsByTagName("tr")[2].id, "second");
  }, "insertRow should insert a tr element before the second row")
});
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.table.row.no_cells",
      "message": "Row 1 of a row group established by a “tbody” element has no cells beginning on it.",
      "severity": "Warning",
      "span": {
        "byte_end": 444,
        "byte_start": 436,
        "col": 5,
        "line": 11
      }
    },
    {
      "category": "Html",
      "code": "html.table.row.no_cells",
      "message": "Row 2 of a row group established by a “tbody” element has no cells beginning on it.",
      "severity": "Warning",
      "span": {
        "byte_end": 444,
        "byte_start": 436,
        "col": 5,
        "line": 11
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/tabular-data/the-table-element/insertRow-method-03.html"
}
```
