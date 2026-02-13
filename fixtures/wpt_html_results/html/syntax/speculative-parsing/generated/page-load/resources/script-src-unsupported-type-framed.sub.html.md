# html/syntax/speculative-parsing/generated/page-load/resources/script-src-unsupported-type-framed.sub.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/syntax/speculative-parsing/generated/page-load/resources/script-src-unsupported-type-framed.sub.html",
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
<title>Speculative parsing, page load (helper file): script-src-unsupported-type</title>
<script src="/common/slow.py?delay=1500"></script>
<script>
  document.write('<plaintext>');
</script>
<!-- speculative case -->
<script src="/html/syntax/speculative-parsing/resources/stash.py?action=put&amp;uuid={{GET[uuid]}}&amp;encodingcheck=&Gbreve;" type=text/plain></script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.script.datablock.src.disallowed",
      "message": "A “script” element with a “type” attribute whose value is neither a JavaScript MIME type, “module”, “importmap”, nor “speculationrules” (i.e., a data block) must not have a “src” attribute.",
      "severity": "Warning",
      "span": {
        "byte_end": 514,
        "byte_start": 371,
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
  "source_name": "html/syntax/speculative-parsing/generated/page-load/resources/script-src-unsupported-type-framed.sub.html"
}
```
