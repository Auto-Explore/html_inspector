# html/rendering/widgets/input-time-content-size.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/widgets/input-time-content-size.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Test: the time field's min-content and max-content sizes should be the same as its automatic size</title>
<link rel="author" title="Daniel Holbert" href="mailto:dholbert@mozilla.com">
<link rel="author" href="https://mozilla.org" title="Mozilla">
<link rel="help" href="https://bugzilla.mozilla.org/show_bug.cgi?id=1695578">
<link rel="match" href="input-time-content-size-ref.html">
<body>
  <input type="time" style="width: min-content">
  <br>
  <input type="time" style="width: max-content">
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
  "source_name": "html/rendering/widgets/input-time-content-size.html"
}
```
