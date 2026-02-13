# html/rendering/non-replaced-elements/tables/table-layout.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/tables/table-layout.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<title>Table layout attribute</title>
<link rel="match" href="table-layout-ref.html">
<meta name="assert"
      content="The layout attribute on table elements should have no effect.">
<table border width=100% layout=fixed>
<tr><td>aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa<td>aaa
</table>
<table border width=100% layout=auto>
<tr><td>aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa<td>aaa
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
  "source_name": "html/rendering/non-replaced-elements/tables/table-layout.html"
}
```
