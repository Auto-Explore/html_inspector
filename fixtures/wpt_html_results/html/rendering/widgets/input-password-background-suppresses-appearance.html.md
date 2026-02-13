# html/rendering/widgets/input-password-background-suppresses-appearance.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/widgets/input-password-background-suppresses-appearance.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<link rel="help" href="https://bugzilla.mozilla.org/show_bug.cgi?id=1895561">
<link rel="author" title="Emilio Cobos Álvarez" href="mailto:emilio@crisal.io">
<link rel="match" href="input-password-background-suppresses-appearance-ref.html">
<title>backgrounds disable native password input appearance</title>
<input type="password" style="background-color: blue">
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
  "source_name": "html/rendering/widgets/input-password-background-suppresses-appearance.html"
}
```
