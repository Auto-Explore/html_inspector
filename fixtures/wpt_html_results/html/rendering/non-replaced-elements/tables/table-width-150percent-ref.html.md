# html/rendering/non-replaced-elements/tables/table-width-150percent-ref.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/tables/table-width-150percent-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Test for capping percentages</title>
<style>
div { width:300px; background:yellow; height:50px; }
table { width:150%; }
td { background:blue; }
</style>
<div>
  <table cellspacing="0" cellpadding="0" border="0">
    <tr><td>parent div float=left</td></tr>
  </table>
</div>
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
  "source_name": "html/rendering/non-replaced-elements/tables/table-width-150percent-ref.html"
}
```
