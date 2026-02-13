# html/rendering/widgets/input-time-content-size-ref.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/widgets/input-time-content-size-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Test reference</title>
<link rel="author" title="Daniel Holbert" href="mailto:dholbert@mozilla.com">
<link rel="author" href="https://mozilla.org" title="Mozilla">
<body>
  <input type="time">
  <br>
  <input type="time">
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
  "source_name": "html/rendering/widgets/input-time-content-size-ref.html"
}
```
