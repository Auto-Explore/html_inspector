# html/semantics/scripting-1/the-template-element/resources/template-contents-body.html

Counts:
- errors: 2
- warnings: 0
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-template-element/resources/template-contents-body.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
    <title>BODY tag inside template</title>
    <link rel="author" title="Aleksei Yu. Semenov" href="mailto:a.semenov@unipro.ru">
<head>
<body>
    <template><body></body></template>
</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.body.already_open",
      "message": "Start tag “body” seen but an element of the same type was already open.",
      "severity": "Error",
      "span": {
        "byte_end": 194,
        "byte_start": 188,
        "col": 15,
        "line": 8
      }
    },
    {
      "category": "Html",
      "code": "html.parser.cannot_recover",
      "message": "Cannot recover after last error. Any further errors will be ignored.",
      "severity": "Error",
      "span": {
        "byte_end": 194,
        "byte_start": 188,
        "col": 15,
        "line": 8
      }
    }
  ],
  "source_name": "html/semantics/scripting-1/the-template-element/resources/template-contents-body.html"
}
```
