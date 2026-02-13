# html/rendering/non-replaced-elements/lists/ul-type-supported-xhtml.xhtml

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/lists/ul-type-supported-xhtml.xhtml",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<?xml version="1.0" encoding="UTF-8"?>
<html xmlns="http://www.w3.org/1999/xhtml">
<head>
<title>ul@type: supported types</title>
<link rel="match" href="ul-type-supported-ref.html"/>
</head>
<body>
<ul type="disc"><li>first disc</li><li>second disc</li></ul>
<ul type="circle"><li>first circle</li><li>second circle</li></ul>
<ul type="square"><li>first square</li><li>second square</li></ul>
<ul type="none"><li>first none</li><li>second none</li></ul>
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
  "source_name": "html/rendering/non-replaced-elements/lists/ul-type-supported-xhtml.xhtml"
}
```
