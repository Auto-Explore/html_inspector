# html/semantics/tabular-data/the-tr-element/rowIndex.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/tabular-data/the-tr-element/rowIndex.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>HTMLTableRowElement.rowIndex</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<script>
test(function() {
  var row = document.createElement("table")
                    .appendChild(document.createElement("div"))
                    .appendChild(document.createElement("tr"));
  assert_equals(row.rowIndex, -1);
});
test(function() {
  var row = document.createElement("table")
                    .appendChild(document.createElement("thead"))
                    .appendChild(document.createElement("tr"));
  assert_equals(row.rowIndex, 0);
});
test(function() {
  var row = document.createElement("table")
                    .appendChild(document.createElement("tbody"))
                    .appendChild(document.createElement("tr"));
  assert_equals(row.rowIndex, 0);
});
test(function() {
  var row = document.createElement("table")
                    .appendChild(document.createElement("tfoot"))
                    .appendChild(document.createElement("tr"));
  assert_equals(row.rowIndex, 0);
});
test(function() {
  var row = document.createElement("table")
                    .appendChild(document.createElement("tr"));
  assert_equals(row.rowIndex, 0);
});
test(function() {
  var row = document.createElementNS("", "table")
                    .appendChild(document.createElement("thead"))
                    .appendChild(document.createElement("tr"));
  assert_equals(row.rowIndex, -1);
});
test(function() {
  var row = document.createElementNS("", "table")
                    .appendChild(document.createElement("tbody"))
                    .appendChild(document.createElement("tr"));
  assert_equals(row.rowIndex, -1);
});
test(function() {
  var row = document.createElementNS("", "table")
                    .appendChild(document.createElement("tfoot"))
                    .appendChild(document.createElement("tr"));
  assert_equals(row.rowIndex, -1);
});
test(function() {
  var row = document.createElementNS("", "table")
                    .appendChild(document.createElement("tr"));
  assert_equals(row.rowIndex, -1);
});
test(function() {
  var row = document.createElement("table")
                    .appendChild(document.createElementNS("", "thead"))
                    .appendChild(document.createElement("tr"));
  assert_equals(row.rowIndex, -1);
});
test(function() {
  var row = document.createElement("table")
                    .appendChild(document.createElementNS("", "tbody"))
                    .appendChild(document.createElement("tr"));
  assert_equals(row.rowIndex, -1);
});
test(function() {
  var row = document.createElement("table")
                    .appendChild(document.createElementNS("", "tfoot"))
                    .appendChild(document.createElement("tr"));
  assert_equals(row.rowIndex, -1);
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
  "source_name": "html/semantics/tabular-data/the-tr-element/rowIndex.html"
}
```
