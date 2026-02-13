# html/semantics/scripting-1/the-template-element/resources/template-contents-nested.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-template-element/resources/template-contents-nested.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
    <title>Contains second template tag inside template tag</title>
    <link rel="author" title="Aleksei Yu. Semenov" href="mailto:a.semenov@unipro.ru">
<body>
    <template>
        <template>
            <div>Inside nested template</div>
        </template>
    </template>
</body>
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
  "source_name": "html/semantics/scripting-1/the-template-element/resources/template-contents-nested.html"
}
```
