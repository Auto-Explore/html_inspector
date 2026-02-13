# html/semantics/text-level-semantics/the-bdo-element/bidi-001.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/text-level-semantics/the-bdo-element/bidi-001.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
<meta charset="UTF-8">
<title>directional type</title>
<meta content="W3C" name="author">
<link rel="match" href="bidi-001-ref.html">
<meta name="assert" content="Test text bidirectionality using the bdo element">
</head>
<body dir='ltr'>
<p>Test passes if there is text 'WERBEH'.</p>
<bdo dir="rtl">HEBREW</bdo>
</body>
</html>
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
  "source_name": "html/semantics/text-level-semantics/the-bdo-element/bidi-001.html"
}
```
