# html/rendering/non-replaced-elements/tables/table-row-direction.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/tables/table-row-direction.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel="match" href="table-row-direction-ref.html">
<title>Table row direction</title>

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

Normal table with LTR and RTL rows:
<table>
  <tr style="direction: ltr">
    <td></td>
    <td class="special"></td>
  </tr>
  <tr style="direction: rtl">
    <td></td>
    <td class="special"></td>
  </tr>
</table>

<hr>

RTL table with LTR and RTL rows:
<table style="direction: rtl">
  <tr style="direction: ltr">
    <td></td>
    <td class="special"></td>
  </tr>
  <tr style="direction: rtl">
    <td></td>
    <td class="special"></td>
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
  "source_name": "html/rendering/non-replaced-elements/tables/table-row-direction.html"
}
```
