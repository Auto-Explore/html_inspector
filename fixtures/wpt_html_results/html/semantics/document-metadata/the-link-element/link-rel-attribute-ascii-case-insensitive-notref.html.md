# html/semantics/document-metadata/the-link-element/link-rel-attribute-ascii-case-insensitive-notref.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/document-metadata/the-link-element/link-rel-attribute-ascii-case-insensitive-notref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>link element rel is ASCII case-insensitive (mismatch reference)</title>
<link rel="stylesheet" href="stylesheet.css">
<p>Test passes if background is not red.</p>
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
  "source_name": "html/semantics/document-metadata/the-link-element/link-rel-attribute-ascii-case-insensitive-notref.html"
}
```
