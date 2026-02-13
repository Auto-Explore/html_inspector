# html/rendering/widgets/input-date-whitespace-pre-2.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/widgets/input-date-whitespace-pre-2.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>

<meta charset="utf-8">
<title>input type=date test</title>
<meta name="assert" content="CSS white-space property does not disrupt rendering of <input type=date>">
<link rel="help" href="https://bugzilla.mozilla.org/show_bug.cgi?id=1940063">
<link rel="match" href="input-date-whitespace-pre-ref.html">

<body style="white-space: pre"><input type="date">
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
  "source_name": "html/rendering/widgets/input-date-whitespace-pre-2.html"
}
```
