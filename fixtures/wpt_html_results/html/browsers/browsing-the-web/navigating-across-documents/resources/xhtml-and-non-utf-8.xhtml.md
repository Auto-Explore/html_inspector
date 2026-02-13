# html/browsers/browsing-the-web/navigating-across-documents/resources/xhtml-and-non-utf-8.xhtml

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/navigating-across-documents/resources/xhtml-and-non-utf-8.xhtml",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<?xml version="1.0" encoding="windows-1250"?>
<html xmlns="http://www.w3.org/1999/xhtml">
<head>
  <meta charset="windows-1250"/>
  <title>A test document used when you need something very non-default</title>
</head>
<body>
</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.meta.charset.mismatch",
      "message": "Internal encoding declaration “windows-1250” disagrees with the actual encoding of the document (“utf-8”).",
      "severity": "Warning",
      "span": {
        "byte_end": 129,
        "byte_start": 99,
        "col": 3,
        "line": 4
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
  "source_name": "html/browsers/browsing-the-web/navigating-across-documents/resources/xhtml-and-non-utf-8.xhtml"
}
```
