# html/semantics/scripting-1/the-script-element/module/dynamic-import/resources/v8-code-cache-iframe.sub.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/module/dynamic-import/resources/v8-code-cache-iframe.sub.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta http-equiv="content-security-policy"
      content="script-src 'nonce-{{GET[nonce]}}'">
<!--
base element to make the base URLs of the Document and the script different
-->
<base href="../">
<script src="resources/v8-code-cache.js?pipe=header(Cache-Control,max-age=1000)"
    nonce="{{GET[nonce]}}" type="{{GET[type]}}"></script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.csp.external_script.blocked",
      "message": "Resource violates Content Security Policy (meta tag): external script “resources/v8-code-cache.js?pipe=header(Cache-Contr” blocked by “script-src” directive.",
      "severity": "Warning",
      "span": {
        "byte_end": 342,
        "byte_start": 213,
        "col": 1,
        "line": 8
      }
    },
    {
      "category": "Html",
      "code": "html.script.datablock.src.disallowed",
      "message": "A “script” element with a “type” attribute whose value is neither a JavaScript MIME type, “module”, “importmap”, nor “speculationrules” (i.e., a data block) must not have a “src” attribute.",
      "severity": "Warning",
      "span": {
        "byte_end": 342,
        "byte_start": 213,
        "col": 1,
        "line": 8
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
  "source_name": "html/semantics/scripting-1/the-script-element/module/dynamic-import/resources/v8-code-cache-iframe.sub.html"
}
```
