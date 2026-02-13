# html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-border-gap-position-relative-ref.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-border-gap-position-relative-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Reference for fieldset border gap</title>
<style>
div { position: relative; top: 25px; width: 100px; height: 50px; background: lime; }
</style>
<p>There should be no red.</p>
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
  "source_name": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-border-gap-position-relative-ref.html"
}
```
