# html/semantics/tabular-data/the-tr-element/insertCell.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/tabular-data/the-tr-element/insertCell.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>HTMLTableRowElement#insertCell</title>
<link rel="author" title="Intel" href="http://www.intel.com/">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<div id="log"></div>

<table>
  <tr id="testTr"></tr>
</table>

<script>

var tr = document.getElementById("testTr");

test(function () {
  var tdEle = tr.insertCell(0);
  assert_equals(tr.cells[0], tdEle);
  assert_equals(tr.cells.length, 1);
}, "HTMLTableRowElement insertCell(0)");

test(function () {
  var tdEle = tr.insertCell(-1);
  assert_equals(tr.cells[tr.cells.length - 1], tdEle);
  assert_equals(tr.cells.length, 2);
}, "HTMLTableRowElement insertCell(-1)");


test(function () {
  var tdEle = tr.insertCell(tr.cells.length);
  assert_equals(tr.cells[tr.cells.length - 1], tdEle);
  assert_equals(tr.cells.length, 3);
}, "HTMLTableRowElement insertCell(cells.length)");

test(function () {
  var tdEle = tr.insertCell();
  assert_equals(tr.cells[tr.cells.length - 1], tdEle);
  assert_equals(tr.cells.length, 4);
}, "HTMLTableRowElement insertCell()");

test(function () {
  assert_throws_dom("IndexSizeError", function () {
    tr.insertCell(-2);
  });
}, "HTMLTableRowElement insertCell(-2)");

test(function () {
  assert_throws_dom("IndexSizeError", function () {
    tr.insertCell(tr.cells.length + 1);
  });
}, "HTMLTableRowElement insertCell(cells.length + 1)");

test(function () {
  var table = document.createElementNS("http://www.w3.org/1999/xhtml", "foo:table")
  var row = table.insertRow(0);
  var cell = row.insertCell(0);

  assert_equals(row.cells[0], cell);
  assert_equals(cell.prefix, null);
}, "HTMLTableRowElement insertCell will not copy table's prefix");

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
        "byte_end": 317,
        "byte_start": 309,
        "col": 1,
        "line": 12
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
  "source_name": "html/semantics/tabular-data/the-tr-element/insertCell.html"
}
```
