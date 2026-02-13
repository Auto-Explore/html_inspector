# html/semantics/forms/the-input-element/date-time-with-trusted-types.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-input-element/date-time-with-trusted-types.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
<link rel="match" href="date-time-with-trusted-types-ref.html">
<meta http-equiv="Content-Security-Policy" content="require-trusted-types-for 'script'">
</head>
<body>
<input type="date"><br>
<input type="month"><br>
<input type="week"><br>
<input type="time"><br>
<input type="datetime-local"><br>
</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.meta.csp.invalid",
      "message": "Bad value “require-trusted-types-for 'script'” for attribute “content” on element “meta”.",
      "severity": "Warning",
      "span": {
        "byte_end": 182,
        "byte_start": 94,
        "col": 1,
        "line": 5
      }
    },
    {
      "category": "Html",
      "code": "html.head.title.missing",
      "message": "Element “head” is missing a required instance of child element “title”.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/forms/the-input-element/date-time-with-trusted-types.html"
}
```
