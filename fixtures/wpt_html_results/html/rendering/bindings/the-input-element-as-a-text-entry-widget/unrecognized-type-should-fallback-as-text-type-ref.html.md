# html/rendering/bindings/the-input-element-as-a-text-entry-widget/unrecognized-type-should-fallback-as-text-type-ref.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/bindings/the-input-element-as-a-text-entry-widget/unrecognized-type-should-fallback-as-text-type-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<title>Unrecognized type should fallback as text type</title>
<body>
  <input type="text">
  <input type="text">
  <input type="text" disabled>
  <input type="text" disabled>
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
  "source_name": "html/rendering/bindings/the-input-element-as-a-text-entry-widget/unrecognized-type-should-fallback-as-text-type-ref.html"
}
```
