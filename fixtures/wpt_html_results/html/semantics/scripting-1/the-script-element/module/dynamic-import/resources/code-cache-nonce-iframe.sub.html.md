# html/semantics/scripting-1/the-script-element/module/dynamic-import/resources/code-cache-nonce-iframe.sub.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/module/dynamic-import/resources/code-cache-nonce-iframe.sub.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta http-equiv="content-security-policy"
      content="script-src 'unsafe-eval' 'nonce-{{GET[nonce]}}'">
<script src="code-cache-nonce.js" nonce="{{GET[nonce]}}"></script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.csp.external_script.blocked",
      "message": "Resource violates Content Security Policy (meta tag): external script “code-cache-nonce.js” blocked by “script-src” directive.",
      "severity": "Warning",
      "span": {
        "byte_end": 181,
        "byte_start": 124,
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
  "source_name": "html/semantics/scripting-1/the-script-element/module/dynamic-import/resources/code-cache-nonce-iframe.sub.html"
}
```
