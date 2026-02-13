# html/rendering/the-details-element/summary-text-decoration-ref.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/the-details-element/summary-text-decoration-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Test reference</title>
<style>
div {
  display: list-item;
  list-style-type: disclosure-closed;
  list-style-position: inside;
  text-decoration: underline;
}
</style>
<div>This text should be underlined.</div>
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
  "source_name": "html/rendering/the-details-element/summary-text-decoration-ref.html"
}
```
