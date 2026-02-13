# html/syntax/speculative-parsing/generated/resources/meta-csp-img-src-none-nonspeculative.sub.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/syntax/speculative-parsing/generated/resources/meta-csp-img-src-none-nonspeculative.sub.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<!-- DO NOT EDIT. This file has been generated. Source:
     /html/syntax/speculative-parsing/tools/generate.py
-->
<meta charset=utf-8>
<title>Speculative parsing, non-speculative (helper file): meta-csp-img-src-none</title>
<!-- non-speculative case -->
<meta http-equiv="Content-Security-Policy" content="script-src 'self' 'unsafe-inline'; img-src 'none'"><img src="/html/syntax/speculative-parsing/resources/stash.py?action=put&amp;uuid={{GET[uuid]}}&amp;encodingcheck=&Gbreve;">
<!-- block the load event for a bit: -->
<script src="/common/slow.py?delay=1500"></script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 499,
        "byte_start": 375,
        "col": 104,
        "line": 8
      }
    },
    {
      "category": "Html",
      "code": "html.csp.image.blocked",
      "message": "Resource violates Content Security Policy (meta tag): image “/html/syntax/speculative-parsing/resources/stash.p” blocked by “img-src” directive.",
      "severity": "Warning",
      "span": {
        "byte_end": 499,
        "byte_start": 375,
        "col": 104,
        "line": 8
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/syntax/speculative-parsing/generated/resources/meta-csp-img-src-none-nonspeculative.sub.html"
}
```
