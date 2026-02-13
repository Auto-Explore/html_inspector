# html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-border-gap-negative-margin.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-border-gap-negative-margin.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>fieldset border gap with negative margin</title>
<link rel=match href=no-red-ref.html>
<style>
fieldset { border:none; border-left: 100px solid red; margin: 0; padding: 0; height:100px; }
legend { padding: 0; margin-left: -100px; width: 100px; height: 100px; transform: rotate(45deg); }
</style>
<p>There should be no red.</p>
<fieldset>
 <legend></legend>
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
  "source_name": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-border-gap-negative-margin.html"
}
```
