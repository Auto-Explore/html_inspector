# html/semantics/scripting-1/the-script-element/module/dynamic-import/propagate-nonce-external-module.html

Counts:
- errors: 0
- warnings: 5
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/module/dynamic-import/propagate-nonce-external-module.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta http-equiv="content-security-policy" content="script-src 'nonce-correct'">
<script nonce="correct" src="/resources/testharness.js"></script>
<script nonce="correct" src="/resources/testharnessreport.js"></script>
<script type="module" nonce="correct" src="./propagate-nonce-external.js"></script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.csp.external_script.blocked",
      "message": "Resource violates Content Security Policy (meta tag): external script “/resources/testharness.js” blocked by “script-src” directive.",
      "severity": "Warning",
      "span": {
        "byte_end": 153,
        "byte_start": 97,
        "col": 1,
        "line": 3
      }
    },
    {
      "category": "Html",
      "code": "html.csp.external_script.blocked",
      "message": "Resource violates Content Security Policy (meta tag): external script “/resources/testharnessreport.js” blocked by “script-src” directive.",
      "severity": "Warning",
      "span": {
        "byte_end": 225,
        "byte_start": 163,
        "col": 1,
        "line": 4
      }
    },
    {
      "category": "Html",
      "code": "html.csp.external_script.blocked",
      "message": "Resource violates Content Security Policy (meta tag): external script “./propagate-nonce-external.js” blocked by “script-src” directive.",
      "severity": "Warning",
      "span": {
        "byte_end": 309,
        "byte_start": 235,
        "col": 1,
        "line": 5
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
  "source_name": "html/semantics/scripting-1/the-script-element/module/dynamic-import/propagate-nonce-external-module.html"
}
```
