# html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend-position-relative.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend-position-relative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>legend position: relative</title>
<link rel=match href=legend-position-relative-ref.html>
<style>
fieldset { border: 100px solid lime; width: 200px; padding: 0; margin: 0 }
legend { position: relative; left: 100px; width: 100px; height: 100px; padding: 0 }
.behind { position: absolute; left: 208px; width: 100px; height: 100px; background: red; z-index: -1 }
</style>
<p>There should be no red.</p>
<div class=behind></div>
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
  "source_name": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend-position-relative.html"
}
```
