# html/semantics/tabular-data/processing-model-1/rowspan-0-quirks.html

Counts:
- errors: 1
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/tabular-data/processing-model-1/rowspan-0-quirks.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!--quirks mode-->
<title>rowspan=0 in quirks mode</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<table>
 <tr>
  <td rowspan="0" id="test-cell">x
  <td id="ref-cell">x
 <tr>
  <td>x
</table>
<script>
const testCell = document.getElementById('test-cell');
const refCell = document.getElementById('ref-cell');

test(() => {
  assert_greater_than(testCell.clientHeight, refCell.clientHeight);
}, 'clientHeight');

test(() => {
  assert_equals(testCell.rowSpan, 0);
}, 'rowSpan');
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.start_tag_without_doctype",
      "message": "Start tag seen without seeing a doctype first. Expected “<!DOCTYPE html>”.",
      "severity": "Error",
      "span": {
        "byte_end": 26,
        "byte_start": 19,
        "col": 1,
        "line": 2
      }
    },
    {
      "category": "Html",
      "code": "html.table.row.width.less_than_first_row",
      "message": "A table row was 1 columns wide, which is less than the column count established by the first row (2).",
      "severity": "Warning",
      "span": {
        "byte_end": 254,
        "byte_start": 246,
        "col": 1,
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
  "source_name": "html/semantics/tabular-data/processing-model-1/rowspan-0-quirks.html"
}
```
