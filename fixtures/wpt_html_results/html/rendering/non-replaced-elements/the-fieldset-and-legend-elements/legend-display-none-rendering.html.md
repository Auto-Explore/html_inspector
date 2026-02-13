# html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend-display-none-rendering.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend-display-none-rendering.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Rendering of display: none legend</title>
<link rel=match href=legend-display-none-rendering-ref.html>
<style>
 fieldset { border: 2em solid lime; width: 0; margin: 0; padding: 0; }
 legend { display: none; background: red; }
</style>
<p>There should be a green box below.</p>
<fieldset>
 <legend>FAIL</legend>
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
  "source_name": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend-display-none-rendering.html"
}
```
