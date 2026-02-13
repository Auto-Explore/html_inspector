# html/semantics/scripting-1/the-template-element/resources/end-template-tag-in-body.html

Counts:
- errors: 3
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-template-element/resources/end-template-tag-in-body.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
    <title>The file contains several &lt;/template&gt; tag in HTML body without start one</title>
    <link rel="author" title="Aleksei Yu. Semenov" href="mailto:a.semenov@unipro.ru">
</head>
<body>
    </template>
    <div>The file contains several &lt;/template&gt; tag in HTML body without start one</div>
    </template></template>
</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “template”.",
      "severity": "Error",
      "span": {
        "byte_end": 244,
        "byte_start": 233,
        "col": 5,
        "line": 8
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “template”.",
      "severity": "Error",
      "span": {
        "byte_end": 354,
        "byte_start": 343,
        "col": 5,
        "line": 10
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “template”.",
      "severity": "Error",
      "span": {
        "byte_end": 365,
        "byte_start": 354,
        "col": 16,
        "line": 10
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
  "source_name": "html/semantics/scripting-1/the-template-element/resources/end-template-tag-in-body.html"
}
```
