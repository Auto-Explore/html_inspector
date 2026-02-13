# html/semantics/scripting-1/the-template-element/resources/html-start-tag.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-template-element/resources/html-start-tag.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html tabindex="5">
<head>
    <title>The file contains html root element with attributes and some in the body</title>
    <link rel="author" title="Sergey G. Grekhovv" href="mailto:sgrekhov@unipro.ru">
</head>
<body>
<template id="tmpl"><html class="htmlClass"></html></template><html id="htmlId" tabindex="5">
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
  "source_name": "html/semantics/scripting-1/the-template-element/resources/html-start-tag.html"
}
```
