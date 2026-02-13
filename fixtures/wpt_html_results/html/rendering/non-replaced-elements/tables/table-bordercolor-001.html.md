# html/rendering/non-replaced-elements/tables/table-bordercolor-001.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/tables/table-bordercolor-001.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Test for table bordercolor attribute behaving like border-color property</title>
<link rel="author" title="Daniel Holbert" href="mailto:dholbert@mozilla.com">
<link rel="author" title="Mozilla" href="https://www.mozilla.org/">
<link rel="help" href="https://html.spec.whatwg.org/multipage/rendering.html#tables-2">
<link rel="match" href="table-bordercolor-001-ref.html">
<meta name="assert" content="bordercolor is treated as a presentation hint, equivalent to setting the border-color property">
<style>
  table { margin: 5px }
</style>
<table bordercolor="red">
  <td>I should not have a border.</td>
</table>
<table style="border-color: red">
  <td>I should not have a border.</td>
</table>
<table bordercolor="red" style="border-width: 10px">
  <td>I should not have a border.</td>
</table>
<table style="border-color: red; border-width: 10px">
  <td>I should not have a border.</td>
</table>
<table bordercolor="lime" style="border-style: solid">
  <td>I should have a border.</td>
</table>
<table style="border-color: lime; border-style: solid">
  <td>I should have a border.</td>
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
  "source_name": "html/rendering/non-replaced-elements/tables/table-bordercolor-001.html"
}
```
