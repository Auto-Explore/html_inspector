# html/semantics/embedded-content/the-img-element/animated-image-update-ref.tentative.html

Counts:
- errors: 1
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/animated-image-update-ref.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<head>
  <title>Reference for Animated image active frame update when passing delay time</title>
</head>

<body>
    <img src="resources/green.png">
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
        "byte_end": 164,
        "byte_start": 133,
        "col": 5,
        "line": 7
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “html”.",
      "severity": "Error",
      "span": {
        "byte_end": 180,
        "byte_start": 173,
        "col": 1,
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
  "source_name": "html/semantics/embedded-content/the-img-element/animated-image-update-ref.tentative.html"
}
```
