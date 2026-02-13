# html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend-block-margins-ref.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend-block-margins-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Reference for legend block margins</title>
<style>
body { margin: 0; }
.fieldset { margin: 2em 1em 1em 1em; border: 1em solid green; }
.legend { position: absolute; margin-top: -1em; margin-left: 1em; background: white; height: 1em; }
.inner { margin: 3em 1em 1em 1em; height: 1em; }
</style>
<p>There should be no red.</p>
<div class=fieldset>
  <div class=legend>X</div>
  <div class=inner>Y</div>
</div>
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
  "source_name": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend-block-margins-ref.html"
}
```
