# html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-transform-translatez-ref.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-transform-translatez-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Reference for Fieldset and transform: translateZ(0)</title>
<style>
fieldset { background: #eee; margin: 0 0 10px; }
</style>
<p>It should say PASS below without anything obscuring the text.</p>

<fieldset>
  <legend>PASS</legend>
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
  "source_name": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-transform-translatez-ref.html"
}
```
