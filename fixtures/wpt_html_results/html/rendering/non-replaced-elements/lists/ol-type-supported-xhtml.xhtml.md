# html/rendering/non-replaced-elements/lists/ol-type-supported-xhtml.xhtml

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/lists/ol-type-supported-xhtml.xhtml",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<?xml version="1.0" encoding="UTF-8"?>
<html xmlns="http://www.w3.org/1999/xhtml">
<head>
<title>ol@type: supported types</title>
<link rel="match" href="ol-type-supported-ref.html"/>
</head>
<body>
<ol type="1"><li>1</li><li>2</li></ol>
<ol type="a"><li>a</li><li>b</li></ol>
<ol type="A"><li>A</li><li>B</li></ol>
<ol type="i"><li>i</li><li>ii</li></ol>
<ol type="I"><li>I</li><li>II</li></ol>
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
  "source_name": "html/rendering/non-replaced-elements/lists/ol-type-supported-xhtml.xhtml"
}
```
