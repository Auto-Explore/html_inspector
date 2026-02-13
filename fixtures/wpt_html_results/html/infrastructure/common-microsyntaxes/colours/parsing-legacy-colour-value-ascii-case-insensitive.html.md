# html/infrastructure/common-microsyntaxes/colours/parsing-legacy-colour-value-ascii-case-insensitive.html

Counts:
- errors: 0
- warnings: 5
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/infrastructure/common-microsyntaxes/colours/parsing-legacy-colour-value-ascii-case-insensitive.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<link rel="help" href="https://html.spec.whatwg.org/#rules-for-parsing-a-legacy-colour-value">
<link rel="match" href="parsing-legacy-colour-value-ascii-case-insensitive-ref.html">
<meta name="assert" content="special legacy color value “transparent” is ASCII case-insensitive">
<link rel="stylesheet" type="text/css" href="/fonts/ahem.css">
<p>This square should be black: <font face="Ahem" color="transparent">X</font>
<p>This square should be black: <font face="Ahem" color="TrAnSpArEnT">X</font>
<p>This square should be blue: <font face="Ahem" color="tranſparent">X</font>
<!--
  Following the rules for parsing a legacy color value should yield a shade of
  blue, because only the following steps apply, not step 4:

  1.    tranſparent
  10.   00a000a0e00
  11.   00a000a0e000
  12.   00a0 00a0 e000 (length = 4)
  15.   00   00   e0
  20.   #0000E0
-->
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
        "byte_end": 455,
        "byte_start": 417,
        "col": 33,
        "line": 7
      }
    },
    {
      "category": "Html",
      "code": "html.element.font.obsolete",
      "message": "The “font” element is obsolete. Use CSS instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 534,
        "byte_start": 496,
        "col": 33,
        "line": 8
      }
    },
    {
      "category": "Html",
      "code": "html.element.font.obsolete",
      "message": "The “font” element is obsolete. Use CSS instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 613,
        "byte_start": 574,
        "col": 32,
        "line": 9
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
  "source_name": "html/infrastructure/common-microsyntaxes/colours/parsing-legacy-colour-value-ascii-case-insensitive.html"
}
```
