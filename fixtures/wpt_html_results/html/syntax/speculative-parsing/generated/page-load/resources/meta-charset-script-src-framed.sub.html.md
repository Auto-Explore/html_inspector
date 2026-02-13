# html/syntax/speculative-parsing/generated/page-load/resources/meta-charset-script-src-framed.sub.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/syntax/speculative-parsing/generated/page-load/resources/meta-charset-script-src-framed.sub.html",
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
<!-- no meta charset -->
<!-- (padding to exceed 1024 bytes processed by the character encoding scanner)                  -->
<!--                                                                                             -->
<!--                                                                                             -->
<!--                                                                                             -->
<!--                                                                                             -->
<!--                                                                                             -->
<!--                                                                                             -->
<!--                                                                                             -->
<!--                                                                                             -->
<!--                                                                                             -->
<title>Speculative parsing, page load (helper file): meta-charset-script-src</title>
<script src="/common/slow.py?delay=1500"></script>
<script>
  document.write('<plaintext>');
</script>
<!-- speculative case -->
<meta charset=windows-1254><script src="/html/syntax/speculative-parsing/resources/stash.py?action=put&amp;uuid={{GET[uuid]}}&amp;encodingcheck=&Gbreve;"></script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.meta.charset_after_1024",
      "message": "A “charset” attribute on a “meta” element found after the first 1024 bytes.",
      "severity": "Warning",
      "span": {
        "byte_end": 1408,
        "byte_start": 1381,
        "col": 1,
        "line": 22
      }
    },
    {
      "category": "I18n",
      "code": "i18n.meta.charset.mismatch",
      "message": "Internal encoding declaration “windows-1254” disagrees with the actual encoding of the document (“utf-8”).",
      "severity": "Warning",
      "span": {
        "byte_end": 1408,
        "byte_start": 1381,
        "col": 1,
        "line": 22
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
  "source_name": "html/syntax/speculative-parsing/generated/page-load/resources/meta-charset-script-src-framed.sub.html"
}
```
