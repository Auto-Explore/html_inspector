# html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-resize.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-resize.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>resize property on fieldset</title>
<link rel="match" href="fieldset-resize-ref.html">
<style>
  fieldset {
    margin: 0;
    border: 1px solid;
    padding: 10px;
    resize: both;
    overflow: scroll;
    width: 100px;
    height: 100px;
  }
</style>
<fieldset></fieldset>
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
  "source_name": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-resize.html"
}
```
