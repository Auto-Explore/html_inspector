# html/rendering/non-replaced-elements/tables/transformed-tbody-tr-collapsed-border.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/tables/transformed-tbody-tr-collapsed-border.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Test for transformed tbody and tr with collapsed borders</title>
<link rel="match" href="transformed-tbody-tr-collapsed-border-ref.html">
<style>
table {
  border-collapse: collapse;
}
tbody, tr {
  transform: translateY(0);
}
td {
  border: 5px solid black;
  width: 100px;
  height: 100px;
}
</style>
Passes if there is a grid containing 2x2 squares.
<table>
  <tbody>
    <tr><td></td><td></td></tr>
    <tr><td></td><td></td></tr>
  </tbody>
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
  "source_name": "html/rendering/non-replaced-elements/tables/transformed-tbody-tr-collapsed-border.html"
}
```
