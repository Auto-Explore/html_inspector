# html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-border-radius-with-alpha.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-border-radius-with-alpha.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Fieldset with a border-radius and non-opaque border-color</title>
<link rel="match" href="fieldset-border-radius-with-alpha-ref.html">
<style>
fieldset {
    background-color: green;
    border: 3px solid rgba(255, 0, 0, 0.9);
    border-radius: 4px;
    height: 100px;
    width: 100px;
}
legend {
    height: 50px;
    width: 50px;
}
div {
    background-color: green;
    height: 110px;
    position: absolute;
    width: 150px;
    top: 70px;
}
</style>
<p>There should be no red.</p>
<fieldset><legend></legend></fieldset>
<div></div>
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
  "source_name": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-border-radius-with-alpha.html"
}
```
