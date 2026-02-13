# html/the-xhtml-syntax/parsing-xhtml-documents/adopt-while-parsing-001-ref.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/the-xhtml-syntax/parsing-xhtml-documents/adopt-while-parsing-001-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Test reference</title>
<style>
  html, body { margin: 0 }
</style>
<iframe src="about:blank"></iframe>
<div>
  PASS
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
  "source_name": "html/the-xhtml-syntax/parsing-xhtml-documents/adopt-while-parsing-001-ref.html"
}
```
