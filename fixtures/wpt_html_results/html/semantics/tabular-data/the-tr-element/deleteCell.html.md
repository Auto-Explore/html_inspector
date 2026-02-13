# html/semantics/tabular-data/the-tr-element/deleteCell.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/tabular-data/the-tr-element/deleteCell.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>HTMLTableRowElement#deleteCell</title>
<link rel="author" title="Intel" href="http://www.intel.com/">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<div id="log"></div>

<table>
  <tr id="testTr">
    <td>ABCDE</td>
    <td>12345</td>
    <td>ABC12</td>
  </tr>
</table>

<script>

var tr = document.getElementById("testTr");

test(function () {
  tr.deleteCell(0);
  assert_equals(tr.cells[0].innerHTML, "12345");
  assert_equals(tr.cells.length, 2);
}, "HTMLTableRowElement deleteCell(0)");

test(function () {
  tr.deleteCell(-1);
  assert_equals(tr.cells[tr.cells.length - 1].innerHTML, "12345");
  assert_equals(tr.cells.length, 1);
}, "HTMLTableRowElement deleteCell(-1)");

test(function () {
  assert_throws_dom("IndexSizeError", function () {
    tr.deleteCell(-2);
  });
}, "HTMLTableRowElement deleteCell(-2)");

test(function () {
  assert_throws_dom("IndexSizeError", function () {
    tr.deleteCell(tr.cells.length);
  });
}, "HTMLTableRowElement deleteCell(cells.length)");

test(function () {
  assert_equals(tr.cells.length, 1);
  tr.deleteCell(-1);
  assert_equals(tr.cells.length, 0);
  tr.deleteCell(-1);
  assert_equals(tr.cells.length, 0);
}, "HTMLTableRowElement deleteCell(-1) with no cells");

test(function () {
  assert_equals(tr.cells.length, 0);
  assert_throws_dom("IndexSizeError", function () {
    tr.deleteCell(0);
  });
}, "HTMLTableRowElement deleteCell(0) with no cells");

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
  "source_name": "html/semantics/tabular-data/the-tr-element/deleteCell.html"
}
```
