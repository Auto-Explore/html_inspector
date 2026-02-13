# html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend-block-margins.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend-block-margins.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>legend block margins</title>
<link rel=match href=legend-block-margins-ref.html>
<style>
 body { margin: 0; }
 fieldset { margin: 1em; border: 1em solid green; padding: 0; background: white; }
 legend { margin: 1em 1em 2em 1em; height: 1em; padding: 0; }
 .inner { margin: 1em; height: 1em; }
 .behind { position: absolute; left: 1em; right: 1em; margin-top: 1em; height: 7em; background: red; z-index: -1; }
</style>
<p>There should be no red.</p>
<div class=behind></div>
<fieldset>
  <legend>X</legend>
  <div class=inner>Y</div>
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
  "source_name": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend-block-margins.html"
}
```
