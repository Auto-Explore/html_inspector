# html/semantics/scripting-1/the-template-element/resources/template-descendant-body.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-template-element/resources/template-descendant-body.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
    <title>Div tag inside template tag</title>
    <link rel="author" title="Sergey G. Grekhov" href="mailto:sgrekhov@unipro.ru">
</head>
<body>
    <template>
        <div>Hello, template</div>
    </template>
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
  "source_name": "html/semantics/scripting-1/the-template-element/resources/template-descendant-body.html"
}
```
