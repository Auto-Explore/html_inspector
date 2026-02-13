# html/syntax/speculative-parsing/generated/resources/picture-source-no-img-nonspeculative.sub.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/syntax/speculative-parsing/generated/resources/picture-source-no-img-nonspeculative.sub.html",
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
<title>Speculative parsing, non-speculative (helper file): picture-source-no-img</title>
<!-- non-speculative case -->
<picture><source srcset="/html/syntax/speculative-parsing/resources/stash.py?action=put&amp;uuid={{GET[uuid]}}&amp;encodingcheck=&Gbreve;"></picture>
<!-- block the load event for a bit: -->
<script src="/common/slow.py?delay=1500"></script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.source.srcset.invalid",
      "message": "Bad value “/html/syntax/speculative-parsing/resources/stash.py?action=put&uuid={{GET[uuid]}}&encodingcheck=Ğ” for attribute “srcset” on element “source”.",
      "severity": "Warning",
      "span": {
        "byte_end": 411,
        "byte_start": 281,
        "col": 10,
        "line": 8
      }
    },
    {
      "category": "Html",
      "code": "html.picture.missing_img",
      "message": "Element “picture” is missing a required instance of child element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 421,
        "byte_start": 411,
        "col": 140,
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
  "source_name": "html/syntax/speculative-parsing/generated/resources/picture-source-no-img-nonspeculative.sub.html"
}
```
