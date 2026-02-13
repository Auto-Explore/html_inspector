# html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend-list-item.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend-list-item.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>legend and dipslay: list-item</title>
<link rel=match href=legend-list-item-ref.html>
<style>
  fieldset { margin: 0; padding: 0; border: none; }
  legend { margin: 0 40px; padding: 0; display: list-item; }
</style>
<p>There should be a bullet point below.</p>
<fieldset>
  <legend>X</legend>
</fieldset>
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
  "source_name": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend-list-item.html"
}
```
