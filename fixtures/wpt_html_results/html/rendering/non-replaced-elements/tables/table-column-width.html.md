# html/rendering/non-replaced-elements/tables/table-column-width.html

Counts:
- errors: 1
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/tables/table-column-width.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<link rel="match" href="table-column-width-ref.html">
<table style="display: block">
  <colgroup style="display: block">
    <col style="border: 1px solid green; display: block" width="0"></col>
  </colgroup>
</table>
<script>
  document.querySelector("col").append("Text");
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “col”.",
      "severity": "Error",
      "span": {
        "byte_end": 210,
        "byte_start": 204,
        "col": 68,
        "line": 5
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
  "source_name": "html/rendering/non-replaced-elements/tables/table-column-width.html"
}
```
