# html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend-position-relative-ref.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend-position-relative-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Reference for legend position: relative</title>
<style>
div { display: inline-block; background: lime; }
.a { width: 100px; height: 200px; }
.b { width: 100px; height: 100px; }
.c { width: 200px; height: 200px; }
</style>
<p>There should be no red.</p>
<div class=a></div><div class=b></div><div class=c></div>
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
  "source_name": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend-position-relative-ref.html"
}
```
