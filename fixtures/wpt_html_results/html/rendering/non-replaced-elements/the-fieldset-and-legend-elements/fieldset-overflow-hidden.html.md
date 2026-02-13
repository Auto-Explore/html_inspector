# html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-overflow-hidden.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-overflow-hidden.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>fieldset and overflow</title>
<link rel=match href=fieldset-overflow-hidden-ref.html>
<style>
fieldset { margin:0; padding: 0; overflow: hidden; border: none; border-top: 1em solid transparent; }
legend { padding: 0; }
</style>
<p>It should say PASS below.</p>
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
  "source_name": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-overflow-hidden.html"
}
```
