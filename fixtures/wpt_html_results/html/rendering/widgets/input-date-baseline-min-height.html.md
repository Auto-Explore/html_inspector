# html/rendering/widgets/input-date-baseline-min-height.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/widgets/input-date-baseline-min-height.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<title>date input baseline should account for min-height</title>
<link rel="match" href="input-date-baseline-min-height-ref.html">
<link rel="help" href="https://bugzilla.mozilla.org/show_bug.cgi?id=1825709">
<link rel="author" title="Emilio Cobos Álvarez" href="mailto:emilio@crisal.io">
<link rel="author" title="Mozilla" href="https://mozilla.org">
<div>
  abc <input type=date style="min-height: 40px"> def
</div>
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
  "source_name": "html/rendering/widgets/input-date-baseline-min-height.html"
}
```
