# html/syntax/speculative-parsing/generated/page-load/resources/img-src-loading-lazy-framed.sub.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/syntax/speculative-parsing/generated/page-load/resources/img-src-loading-lazy-framed.sub.html",
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
<title>Speculative parsing, page load (helper file): img-src-loading-lazy</title>
<script src="/common/slow.py?delay=1500"></script>
<script>
  document.write('<plaintext>');
</script>
<!-- speculative case -->
<img src="/html/syntax/speculative-parsing/resources/stash.py?action=put&amp;uuid={{GET[uuid]}}&amp;encodingcheck=&Gbreve;" loading=lazy>
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
        "byte_end": 501,
        "byte_start": 364,
        "col": 1,
        "line": 12
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
  "source_name": "html/syntax/speculative-parsing/generated/page-load/resources/img-src-loading-lazy-framed.sub.html"
}
```
