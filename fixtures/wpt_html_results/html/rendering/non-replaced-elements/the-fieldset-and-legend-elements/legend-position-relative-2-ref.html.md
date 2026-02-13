# html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend-position-relative-2-ref.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend-position-relative-2-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Reference for legend position: relative</title>
<style>
#fieldset2 {
  background: lime;
  border: 2px solid lime;
  width: 200px;
  padding: 0;
  margin: 0;
}
#legend2 {
  background: #00ffff;
}
</style>
<p>"Legend" should be shown.</p>
<fieldset id="fieldset2"><legend id="legend2">Legend</legend></fieldset>
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
  "source_name": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend-position-relative-2-ref.html"
}
```
