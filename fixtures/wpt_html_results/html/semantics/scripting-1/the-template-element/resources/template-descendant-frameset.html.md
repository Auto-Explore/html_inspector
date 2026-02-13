# html/semantics/scripting-1/the-template-element/resources/template-descendant-frameset.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-template-element/resources/template-descendant-frameset.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
    <title>Template tag inside frameset</title>
    <link rel="author" title="Aleksei Yu. Semenov" href="mailto:a.semenov@unipro.ru">
</head>
<frameset>
    <template>
        <div>Hello, template</div>
    </template>
</frameset>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.element.frameset.obsolete",
      "message": "The “frameset” element is obsolete. Use the “iframe” element and CSS instead, or use server-side includes.",
      "severity": "Warning",
      "span": {
        "byte_end": 182,
        "byte_start": 172,
        "col": 1,
        "line": 7
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
  "source_name": "html/semantics/scripting-1/the-template-element/resources/template-descendant-frameset.html"
}
```
