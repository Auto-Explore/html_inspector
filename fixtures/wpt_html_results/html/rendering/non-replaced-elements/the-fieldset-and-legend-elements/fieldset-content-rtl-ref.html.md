# html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-content-rtl-ref.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-content-rtl-ref.html",
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
<style>
.control {
  background: blue;
  width: 200px;
  height: 1em;
}
.container {
  border: 2px groove ThreeDFace;
  margin: 0;
  padding: 1em;
}
</style>
</head>
<body>
<div class="container">
  <label>Label</label>
  <div class="control" id="ctrl-d"></div>
</div>
</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.head.title.missing",
      "message": "Element “head” is missing a required instance of child element “title”.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-content-rtl-ref.html"
}
```
