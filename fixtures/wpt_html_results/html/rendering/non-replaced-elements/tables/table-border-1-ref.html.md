# html/rendering/non-replaced-elements/tables/table-border-1-ref.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/tables/table-border-1-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<title>Table borders</title>
<style>
table {
  border-width: 1px;
  border-style: outset;
}
td {
  border-width: 1px;
  border-style: inset;
}
</style>
<table>
<tr><td>Test
</table>
<table>
<tr><td>Test
</table>
<table>
<tr><td>Test
</table>
<table>
<tr><td>Test
</table>
<table>
<tr><td>Test
</table>
<table>
<tr><td>Test
</table>
<table>
<tr><td>Test
</table>
<table>
<tr><td>Test
</table>
<table>
<tr><td>Test
</table>
<table>
<tr><td>Test
</table>
<table>
<tr><td>Test
</table>
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
  "source_name": "html/rendering/non-replaced-elements/tables/table-border-1-ref.html"
}
```
