# html/semantics/embedded-content/the-img-element/content-type-with-parameters.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/content-type-with-parameters.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
    <title>SVG is recognized even when the 'Content-Type' header includes parameters</title>
    <link rel="author" title="Mukilan Thiyagrajan" href="mailto:mukilan@igalia.com">
    <link rel="match" href="content-type-with-parameters-ref.html">
</head>
<body>
    <img src="svg-for-content-type-with-parameters.svg">
</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 347,
        "byte_start": 295,
        "col": 5,
        "line": 9
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/embedded-content/the-img-element/content-type-with-parameters.html"
}
```
