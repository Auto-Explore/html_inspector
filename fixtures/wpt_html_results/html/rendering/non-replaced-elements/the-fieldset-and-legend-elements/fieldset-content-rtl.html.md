# html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-content-rtl.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-content-rtl.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html dir="rtl">
<head>
<meta charset="utf-8">
<title>crbug.com/1130174; Non-auto-width block should be right-aligned in an RTL fieldset</title>
<link rel="match" href="fieldset-content-rtl-ref.html">
<style>
.control {
  background: blue;
  width: 200px;
  height: 1em;
}
fieldset {
  border: 2px groove ThreeDFace;
  margin: 0;
  padding: 1em;
}
</style>
</head>
<body>
<fieldset>
  <label>Label</label>
  <div class="control"></div>
</fieldset>
</body>
</html>
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
  "source_name": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-content-rtl.html"
}
```
