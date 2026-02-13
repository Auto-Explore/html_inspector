# html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-border-gap-position-relative.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-border-gap-position-relative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>fieldset border gap</title>
<link rel=match href=fieldset-border-gap-position-relative-ref.html>
<style>
fieldset, legend { margin: 0; padding: 0; }
fieldset { border: none; border-top: 100px solid red; width: 100px; }
legend { width: 100px; height: 50px; background: lime; }
</style>
<p>There should be no red.</p>
<fieldset><legend></legend></fieldset>
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
  "source_name": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-border-gap-position-relative.html"
}
```
