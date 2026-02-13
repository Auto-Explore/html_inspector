# html/syntax/speculative-parsing/generated/page-load/resources/script-src-integrity-framed.sub.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/syntax/speculative-parsing/generated/page-load/resources/script-src-integrity-framed.sub.html",
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
<title>Speculative parsing, page load (helper file): script-src-integrity</title>
<script src="/common/slow.py?delay=1500"></script>
<script>
  document.write('<plaintext>');
</script>
<!-- speculative case -->
<script src="/html/syntax/speculative-parsing/resources/stash.py?action=put&amp;uuid={{GET[uuid]}}&amp;encodingcheck=&Gbreve;" integrity="sha384-OLBgp1GsljhM2TJ+sbHjaiH9txEUvgdDTAzHv2P24donTt6/529l+9Ua0vFImLlb"></script>
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
  "source_name": "html/syntax/speculative-parsing/generated/page-load/resources/script-src-integrity-framed.sub.html"
}
```
