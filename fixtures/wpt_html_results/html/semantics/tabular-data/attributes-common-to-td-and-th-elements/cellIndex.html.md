# html/semantics/tabular-data/attributes-common-to-td-and-th-elements/cellIndex.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/tabular-data/attributes-common-to-td-and-th-elements/cellIndex.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<title>HTMLTableCellElement.cellIndex</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<script>
test(function() {
  var th = document.createElement("th");
  assert_true("cellIndex" in th, '"cellIndex" in th');
  var td = document.createElement("td");
  assert_true("cellIndex" in td, '"cellIndex" in td');
}, "cellIndex should exist.")
test(function() {
  var th = document.createElement("th");
  assert_equals(th.cellIndex, -1);
  var td = document.createElement("td");
  assert_equals(td.cellIndex, -1);
}, "For cells without a parent, cellIndex should be -1.")
test(function() {
  var table = document.createElement("table");
  var th = table.appendChild(document.createElement("th"));
  assert_equals(th.cellIndex, -1);
  var td = table.appendChild(document.createElement("td"));
  assert_equals(td.cellIndex, -1);
}, "For cells whose parent is not a tr, cellIndex should be -1.")
test(function() {
  var tr = document.createElementNS("", "tr");
  var th = tr.appendChild(document.createElement("th"));
  assert_equals(th.cellIndex, -1);
  var td = tr.appendChild(document.createElement("td"));
  assert_equals(td.cellIndex, -1);
}, "For cells whose parent is not a HTML tr, cellIndex should be -1.")
test(function() {
  var tr = document.createElement("tr");
  var th = tr.appendChild(document.createElement("th"));
  assert_equals(th.cellIndex, 0);
  var td = tr.appendChild(document.createElement("td"));
  assert_equals(td.cellIndex, 1);
}, "For cells whose parent is a tr, cellIndex should be the index.")
test(function() {
  var tr = document.createElement("tr");
  var th = tr.appendChild(document.createElement("th"));
  assert_equals(th.cellIndex, 0);
  tr.appendChild(document.createElement("div"));
  tr.appendChild(document.createTextNode("Hello World"));
  var td = tr.appendChild(document.createElement("td"));
  assert_equals(td.cellIndex, 1)
}, "For cells whose parent is a tr with non td/th sibling, cellIndex should skip those non td/th siblings.")
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
  "source_name": "html/semantics/tabular-data/attributes-common-to-td-and-th-elements/cellIndex.html"
}
```
