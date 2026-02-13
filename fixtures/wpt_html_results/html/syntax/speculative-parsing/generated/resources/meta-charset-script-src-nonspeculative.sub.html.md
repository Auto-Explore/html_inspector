# html/syntax/speculative-parsing/generated/resources/meta-charset-script-src-nonspeculative.sub.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/syntax/speculative-parsing/generated/resources/meta-charset-script-src-nonspeculative.sub.html",
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
<title>Speculative parsing, non-speculative (helper file): meta-charset-script-src</title>
<!-- non-speculative case -->
<!-- no meta charset --><script src="/html/syntax/speculative-parsing/resources/stash.py?action=put&amp;uuid={{GET[uuid]}}&amp;encodingcheck=&Gbreve;"></script>
<!-- block the load event for a bit: -->
<script src="/common/slow.py?delay=1500"></script>
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
  "source_name": "html/syntax/speculative-parsing/generated/resources/meta-charset-script-src-nonspeculative.sub.html"
}
```
