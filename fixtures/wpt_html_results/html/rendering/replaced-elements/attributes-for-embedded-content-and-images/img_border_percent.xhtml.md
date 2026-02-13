# html/rendering/replaced-elements/attributes-for-embedded-content-and-images/img_border_percent.xhtml

Counts:
- errors: 0
- warnings: 5
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/replaced-elements/attributes-for-embedded-content-and-images/img_border_percent.xhtml",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<html xmlns='http://www.w3.org/1999/xhtml'>
<head>
<title>IMG - Border= value in percent</title>
<link rel="match" href="img_border-ref.xhtml"/>
</head>
<body>
<p><img src="../../../../../images/blue.png" border="0%"/></p>
<p><img src="../../../../../images/blue.png" border="50%"/></p>
</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.img.border.obsolete",
      "message": "The “border” attribute on the “img” element is obsolete. Consider specifying “img { border: 0; }” in CSS instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 218,
        "byte_start": 163,
        "col": 4,
        "line": 7
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 218,
        "byte_start": 163,
        "col": 4,
        "line": 7
      }
    },
    {
      "category": "Html",
      "code": "html.img.border.obsolete",
      "message": "The “border” attribute on the “img” element is obsolete. Consider specifying “img { border: 0; }” in CSS instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 282,
        "byte_start": 226,
        "col": 4,
        "line": 8
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 282,
        "byte_start": 226,
        "col": 4,
        "line": 8
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
  "source_name": "html/rendering/replaced-elements/attributes-for-embedded-content-and-images/img_border_percent.xhtml"
}
```
