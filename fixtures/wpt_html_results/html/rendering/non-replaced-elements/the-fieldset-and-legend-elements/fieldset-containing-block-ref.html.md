# html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-containing-block-ref.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-containing-block-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Reference for fieldset containing block</title>
<style>
p { margin: 0; height: 100px }
.div1 { position: absolute; top: 108px; width: 100px; height: 100px; background: lime; }
.div2 { position: absolute; top: 158px; width: 200px; height: 100px; background: lime; }
</style>
<p>There should be no red.</p>
<div class="div1"></div>
<div class="div2"></div>
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
  "source_name": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-containing-block-ref.html"
}
```
