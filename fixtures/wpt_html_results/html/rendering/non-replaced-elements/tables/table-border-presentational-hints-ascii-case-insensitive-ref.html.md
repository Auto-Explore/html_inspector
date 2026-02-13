# html/rendering/non-replaced-elements/tables/table-border-presentational-hints-ascii-case-insensitive-ref.html

Counts:
- errors: 0
- warnings: 5
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/tables/table-border-presentational-hints-ascii-case-insensitive-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<link rel="stylesheet" type="text/css" href="/fonts/ahem.css">
<style>
table {
  display: inline-table;
  font-family: Ahem;
}
table, table * {
  border-width: 3px;
}
</style>
<p>For every three tables below, the first two should have borders between some cells, but not the third:
<table rules="rows"><tr><td>X<tr><td>X<tr><td>X</table>
<table rules="rows"><tr><td>X<tr><td>X<tr><td>X</table>
<table><tr><td>X<tr><td>X<tr><td>X</table>
<br><br>
<table rules="cols"><tr><td>X<td>X<td>X</table>
<table rules="cols"><tr><td>X<td>X<td>X</table>
<table><tr><td>X<td>X<td>X</table>
<br><br>
<table rules="groups"><colgroup span="2"><colgroup><tr><td>X<td>X<td>X</table>
<table rules="groups"><colgroup span="2"><colgroup><tr><td>X<td>X<td>X</table>
<table><colgroup span="2"><colgroup><tr><td>X<td>X<td>X</table>
<br><br>
<p>For every three tables below, the first two should have borders on some edges, but not the third:
<table frame="hsides"><tr><td>X</table>
<table frame="hsides"><tr><td>X</table>
<table><tr><td>X</table>
<br><br>
<table frame="lhs"><tr><td>X</table>
<table frame="lhs"><tr><td>X</table>
<table><tr><td>X</table>
<br><br>
<table frame="rhs"><tr><td>X</table>
<table frame="rhs"><tr><td>X</table>
<table><tr><td>X</table>
<br><br>
<table frame="vsides"><tr><td>X</table>
<table frame="vsides"><tr><td>X</table>
<table><tr><td>X</table>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.table.row.width.exceeds_col_markup",
      "message": "A table row was 3 columns wide and exceeded the column count established using column markup (2).",
      "severity": "Warning",
      "span": {
        "byte_end": 703,
        "byte_start": 695,
        "col": 71,
        "line": 22
      }
    },
    {
      "category": "Html",
      "code": "html.table.row.width.exceeds_col_markup",
      "message": "A table row was 3 columns wide and exceeded the column count established using column markup (2).",
      "severity": "Warning",
      "span": {
        "byte_end": 782,
        "byte_start": 774,
        "col": 71,
        "line": 23
      }
    },
    {
      "category": "Html",
      "code": "html.table.row.width.exceeds_col_markup",
      "message": "A table row was 3 columns wide and exceeded the column count established using column markup (2).",
      "severity": "Warning",
      "span": {
        "byte_end": 846,
        "byte_start": 838,
        "col": 56,
        "line": 24
      }
    },
    {
      "category": "Html",
      "code": "html.head.title.missing",
      "message": "Element “head” is missing a required instance of child element “title”.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/rendering/non-replaced-elements/tables/table-border-presentational-hints-ascii-case-insensitive-ref.html"
}
```
