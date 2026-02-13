# html/semantics/tabular-data/the-tbody-element/deleteRow.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/tabular-data/the-tbody-element/deleteRow.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>HTMLTableSectionElement#deleteRow</title>
<link rel="author" title="Intel" href="http://www.intel.com/">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<div id ="log"></div>

<table>
  <tbody id="testBody">
    <tr><td>ABCDEF</td></tr>
    <tr><td>12345</td></tr>
    <tr><td>ABC12345DEF</td></tr>
  </tbody>
</table>

<script>

var tbody = document.getElementById("testBody");

test(function () {
  tbody.deleteRow(0);
  assert_equals(tbody.rows.length, 2);
  assert_equals(tbody.rows[0].childNodes[0].innerHTML, "12345");
}, "HTMLTableSectionElement deleteRow(0)");

test(function () {
  tbody.deleteRow(-1);
  assert_equals(tbody.rows.length, 1);
  assert_equals(tbody.rows[0].childNodes[0].innerHTML, "12345");
}, "HTMLTableSectionElement deleteRow(-1)");

test(function () {
  assert_throws_dom("IndexSizeError", function () {
    tbody.deleteRow(tbody.rows.length);
  });
}, "HTMLTableSectionElement deleteRow(rows.length)");

test(function () {
  assert_throws_dom("IndexSizeError", function () {
    tbody.deleteRow(-2);
  });
}, "HTMLTableSectionElement deleteRow(-2)");

test(function () {
  assert_equals(tbody.rows.length, 1);
  tbody.deleteRow(-1);
  assert_equals(tbody.rows.length, 0);
  tbody.deleteRow(-1);
  assert_equals(tbody.rows.length, 0);
}, "HTMLTableSectionElement deleteRow(-1) with no rows");

test(function () {
  assert_equals(tbody.rows.length, 0);
  assert_throws_dom("IndexSizeError", function () {
    tbody.deleteRow(0);
  });
}, "HTMLTableSectionElement deleteRow(0) with no rows");

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
  "source_name": "html/semantics/tabular-data/the-tbody-element/deleteRow.html"
}
```
