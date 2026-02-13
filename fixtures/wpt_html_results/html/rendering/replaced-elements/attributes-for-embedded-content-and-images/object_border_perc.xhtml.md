# html/rendering/replaced-elements/attributes-for-embedded-content-and-images/object_border_perc.xhtml

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/replaced-elements/attributes-for-embedded-content-and-images/object_border_perc.xhtml",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<html xmlns='http://www.w3.org/1999/xhtml'>
<head>
<title>OBJECT - border=value in %</title>
<link rel="match" href="object_border-ref.xhtml"/>
</head>
<body>
<p><object data="../../../../images/blue.png" type="image/png" border="50%"></object></p>
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
  "source_name": "html/rendering/replaced-elements/attributes-for-embedded-content-and-images/object_border_perc.xhtml"
}
```
