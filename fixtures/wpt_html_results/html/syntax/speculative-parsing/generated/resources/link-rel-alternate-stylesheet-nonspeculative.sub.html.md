# html/syntax/speculative-parsing/generated/resources/link-rel-alternate-stylesheet-nonspeculative.sub.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/syntax/speculative-parsing/generated/resources/link-rel-alternate-stylesheet-nonspeculative.sub.html",
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
<title>Speculative parsing, non-speculative (helper file): link-rel-alternate-stylesheet</title>
<!-- non-speculative case -->
<link rel="alternate stylesheet" href="/html/syntax/speculative-parsing/resources/stash.py?action=put&amp;uuid={{GET[uuid]}}&amp;encodingcheck=&Gbreve;">
<!-- block the load event for a bit: -->
<script src="/common/slow.py?delay=1500"></script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.link.alternate_stylesheet.title.required",
      "message": "A “link” element with a “rel” attribute that contains both the values “alternate” and “stylesheet” must have a “title” attribute with a non-empty value.",
      "severity": "Warning",
      "span": {
        "byte_end": 433,
        "byte_start": 280,
        "col": 1,
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
  "source_name": "html/syntax/speculative-parsing/generated/resources/link-rel-alternate-stylesheet-nonspeculative.sub.html"
}
```
