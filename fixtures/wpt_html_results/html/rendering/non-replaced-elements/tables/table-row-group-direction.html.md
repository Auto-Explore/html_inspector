# html/rendering/non-replaced-elements/tables/table-row-group-direction.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/tables/table-row-group-direction.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel="match" href="table-row-group-direction-ref.html">
<title>Table row-group direction</title>

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

Normal table with LTR and RTL row groups:
<table>
  <tbody style="direction: ltr">
    <tr>
      <td></td>
      <td class="special"></td>
    </tr>
  </tbody>
  <tbody style="direction: rtl">
    <tr>
      <td></td>
      <td class="special"></td>
    </tr>
  </tbody>
</table>

<hr>

RTL table with LTR and RTL row groups:
<table style="direction: rtl">
  <tbody style="direction: ltr">
    <tr>
      <td></td>
      <td class="special"></td>
    </tr>
  </tbody>
  <tbody style="direction: rtl">
    <tr>
      <td></td>
      <td class="special"></td>
    </tr>
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
  "source_name": "html/rendering/non-replaced-elements/tables/table-row-group-direction.html"
}
```
