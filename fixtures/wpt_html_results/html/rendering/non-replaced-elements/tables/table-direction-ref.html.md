# html/rendering/non-replaced-elements/tables/table-direction-ref.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/tables/table-direction-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Table direction</title>

<style>
  table {
    border-collapse: collapse;
  }

  td {
    border: 2px solid black;
    width: 20px;
    height: 20px;
  }

  td.special {
    border-left: 5px solid green;
    border-right: 5px solid blue;
  }
</style>

Normal table:
<table>
  <tr>
    <td></td>
    <td class="special"></td>
  </tr>
</table>

<hr>

RTL table:
<table>
  <tr>
    <td class="special"></td>
    <td></td>
  </tr>
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
  "source_name": "html/rendering/non-replaced-elements/tables/table-direction-ref.html"
}
```
