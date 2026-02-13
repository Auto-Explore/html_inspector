# html/syntax/speculative-parsing/generated/resources/math-font-face-script-src-nonspeculative.sub.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/syntax/speculative-parsing/generated/resources/math-font-face-script-src-nonspeculative.sub.html",
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
<title>Speculative parsing, non-speculative (helper file): math-font-face-script-src</title>
<!-- non-speculative case -->
<math><font face><script src="/html/syntax/speculative-parsing/resources/stash.py?action=put&amp;uuid={{GET[uuid]}}&amp;encodingcheck=&Gbreve;"></script></font></math>
<!-- block the load event for a bit: -->
<script src="/common/slow.py?delay=1500"></script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.mathml.html_start_tag_in_foreign_namespace",
      "message": "HTML start tag “font” in a foreign namespace context.",
      "severity": "Warning",
      "span": {
        "byte_end": 293,
        "byte_start": 282,
        "col": 7,
        "line": 8
      }
    },
    {
      "category": "Html",
      "code": "html.element.font.obsolete",
      "message": "The “font” element is obsolete. Use CSS instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 293,
        "byte_start": 282,
        "col": 7,
        "line": 8
      }
    },
    {
      "category": "Html",
      "code": "html.mathml.html_start_tag_in_foreign_namespace",
      "message": "HTML start tag “script” in a foreign namespace context.",
      "severity": "Warning",
      "span": {
        "byte_end": 420,
        "byte_start": 293,
        "col": 18,
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
  "source_name": "html/syntax/speculative-parsing/generated/resources/math-font-face-script-src-nonspeculative.sub.html"
}
```
