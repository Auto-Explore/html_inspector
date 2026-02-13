# html/infrastructure/common-microsyntaxes/colours/parsing-legacy-colour-value-ascii-case-insensitive-ref.html

Counts:
- errors: 0
- warnings: 5
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/infrastructure/common-microsyntaxes/colours/parsing-legacy-colour-value-ascii-case-insensitive-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<link rel="stylesheet" type="text/css" href="/fonts/ahem.css">
<p>This square should be black: <font face="Ahem">X</font>
<p>This square should be black: <font face="Ahem">X</font>
<p>This square should be blue: <font face="Ahem" color="#0000E0">X</font>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.element.font.obsolete",
      "message": "The “font” element is obsolete. Use CSS instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 152,
        "byte_start": 134,
        "col": 33,
        "line": 4
      }
    },
    {
      "category": "Html",
      "code": "html.element.font.obsolete",
      "message": "The “font” element is obsolete. Use CSS instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 211,
        "byte_start": 193,
        "col": 33,
        "line": 5
      }
    },
    {
      "category": "Html",
      "code": "html.element.font.obsolete",
      "message": "The “font” element is obsolete. Use CSS instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 285,
        "byte_start": 251,
        "col": 32,
        "line": 6
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
  "source_name": "html/infrastructure/common-microsyntaxes/colours/parsing-legacy-colour-value-ascii-case-insensitive-ref.html"
}
```
