# html/rendering/non-replaced-elements/tables/tr-transform-and-will-change.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/tables/tr-transform-and-will-change.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel="match" href="tr-transform-and-will-change-ref.html">
<style>
tbody { background: green; }
td { width: 100px; height: 100px; }
tr { transform: translateX(5px); will-change: transform; }
</style>
<table>
  <tbody>
    <tr><td></td></tr>
    <tr><td></td></tr>
  </tbody>
</table>
There should be 2 green boxes above.
```

```json
{
  "messages": [
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
  "source_name": "html/rendering/non-replaced-elements/tables/tr-transform-and-will-change.html"
}
```
