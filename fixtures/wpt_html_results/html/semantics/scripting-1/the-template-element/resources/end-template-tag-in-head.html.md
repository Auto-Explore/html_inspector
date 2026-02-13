# html/semantics/scripting-1/the-template-element/resources/end-template-tag-in-head.html

Counts:
- errors: 4
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-template-element/resources/end-template-tag-in-head.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
    </template>
    <title>The file contains several &lt;/template&gt; tag in HTML head without start one</title>
    </template></template>
    <link rel="author" title="Aleksei Yu. Semenov" href="mailto:a.semenov@unipro.ru">
    </template>
</head>
<body>
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
        "byte_end": 45,
        "byte_start": 34,
        "col": 5,
        "line": 4
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “template”.",
      "severity": "Error",
      "span": {
        "byte_end": 159,
        "byte_start": 148,
        "col": 5,
        "line": 6
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “template”.",
      "severity": "Error",
      "span": {
        "byte_end": 170,
        "byte_start": 159,
        "col": 16,
        "line": 6
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “template”.",
      "severity": "Error",
      "span": {
        "byte_end": 272,
        "byte_start": 261,
        "col": 5,
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
  "source_name": "html/semantics/scripting-1/the-template-element/resources/end-template-tag-in-head.html"
}
```
