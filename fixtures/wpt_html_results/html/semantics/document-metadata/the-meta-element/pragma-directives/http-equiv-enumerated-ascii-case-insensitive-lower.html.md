# html/semantics/document-metadata/the-meta-element/pragma-directives/http-equiv-enumerated-ascii-case-insensitive-lower.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/document-metadata/the-meta-element/pragma-directives/http-equiv-enumerated-ascii-case-insensitive-lower.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<meta http-equiv="content-security-policy" content="script-src 'self'">
<script>inline = true;</script>
<script src="http-equiv-enumerated-ascii-case-insensitive-message.js"></script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.csp.inline_script.blocked",
      "message": "Inline script violates Content Security Policy (meta tag): blocked by “script-src” directive (missing “‘unsafe-inline’” or nonce/hash).",
      "severity": "Warning",
      "span": {
        "byte_end": 119,
        "byte_start": 111,
        "col": 1,
        "line": 4
      }
    },
    {
      "category": "Html",
      "code": "html.head.title.missing",
      "message": "Element “head” is missing a required instance of child element “title”.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/document-metadata/the-meta-element/pragma-directives/http-equiv-enumerated-ascii-case-insensitive-lower.html"
}
```
