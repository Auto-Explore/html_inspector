# html/rendering/non-replaced-elements/tables/table-border-presentational-hints-ascii-case-insensitive.html

Counts:
- errors: 0
- warnings: 5
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/tables/table-border-presentational-hints-ascii-case-insensitive.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<link rel="help" href="https://html.spec.whatwg.org/#tables-2:presentational-hints">
<link rel="help" href="https://drafts.csswg.org/selectors-4/#attribute-case">
<link rel="match" href="table-border-presentational-hints-ascii-case-insensitive-ref.html">
<meta name="assert" content="@rules + @frame values are ASCII case-insensitive">
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
<table rules="RoWs"><tr><td>X<tr><td>X<tr><td>X</table>
<table rules="rowſ"><tr><td>X<tr><td>X<tr><td>X</table>
<br><br>
<table rules="cols"><tr><td>X<td>X<td>X</table>
<table rules="CoLs"><tr><td>X<td>X<td>X</table>
<table rules="colſ"><tr><td>X<td>X<td>X</table>
<br><br>
<table rules="groups"><colgroup span="2"><colgroup><tr><td>X<td>X<td>X</table>
<table rules="GrOuPs"><colgroup span="2"><colgroup><tr><td>X<td>X<td>X</table>
<table rules="groupſ"><colgroup span="2"><colgroup><tr><td>X<td>X<td>X</table>
<br><br>
<p>For every three tables below, the first two should have borders on some edges, but not the third:
<table frame="hsides"><tr><td>X</table>
<table frame="HsIdEs"><tr><td>X</table>
<table frame="hſideſ"><tr><td>X</table>
<br><br>
<table frame="lhs"><tr><td>X</table>
<table frame="LhS"><tr><td>X</table>
<table frame="lhſ"><tr><td>X</table>
<br><br>
<table frame="rhs"><tr><td>X</table>
<table frame="RhS"><tr><td>X</table>
<table frame="rhſ"><tr><td>X</table>
<br><br>
<table frame="vsides"><tr><td>X</table>
<table frame="VsIdEs"><tr><td>X</table>
<table frame="vſideſ"><tr><td>X</table>
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
        "byte_end": 1067,
        "byte_start": 1059,
        "col": 71,
        "line": 26
      }
    },
    {
      "category": "Html",
      "code": "html.table.row.width.exceeds_col_markup",
      "message": "A table row was 3 columns wide and exceeded the column count established using column markup (2).",
      "severity": "Warning",
      "span": {
        "byte_end": 1146,
        "byte_start": 1138,
        "col": 71,
        "line": 27
      }
    },
    {
      "category": "Html",
      "code": "html.table.row.width.exceeds_col_markup",
      "message": "A table row was 3 columns wide and exceeded the column count established using column markup (2).",
      "severity": "Warning",
      "span": {
        "byte_end": 1226,
        "byte_start": 1218,
        "col": 72,
        "line": 28
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
  "source_name": "html/rendering/non-replaced-elements/tables/table-border-presentational-hints-ascii-case-insensitive.html"
}
```
