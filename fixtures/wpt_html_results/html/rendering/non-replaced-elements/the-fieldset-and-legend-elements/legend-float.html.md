# html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend-float.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend-float.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>floated legend should not disappear</title>
<link rel=match href=legend-float-ref.html>
<style>
  fieldset { margin: 0; padding: 0; border: none; width: 100px; height: 50px; background: red; }
  legend { width: 100px; height: 50px; background: lime; padding: 0; }
  .left { float: left; }
  .right { float: right; }
</style>
<p>There should be no red.</p>
<fieldset><legend class=left></legend></fieldset>
<fieldset><legend class=right></legend></fieldset>
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
  "source_name": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend-float.html"
}
```
