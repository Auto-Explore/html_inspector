# html/syntax/charset/after-bogus.html

Counts:
- errors: 3
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/syntax/charset/after-bogus.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<head>
<link rel="match" href="references/after-bogus-ref.html">
<bogus><meta charset="windows-1251">
</head>
<body>
<p>Meta after <code>bogus</code>.</p>
<p>Test: �</p>
<p>If &#x0436;, meta takes effect</p>
</body>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “head”.",
      "severity": "Error",
      "span": {
        "byte_end": 125,
        "byte_start": 118,
        "col": 1,
        "line": 5
      }
    },
    {
      "category": "Html",
      "code": "html.parser.body.already_open",
      "message": "Start tag “body” seen but an element of the same type was already open.",
      "severity": "Error",
      "span": {
        "byte_end": 132,
        "byte_start": 126,
        "col": 1,
        "line": 6
      }
    },
    {
      "category": "Html",
      "code": "html.parser.cannot_recover",
      "message": "Cannot recover after last error. Any further errors will be ignored.",
      "severity": "Error",
      "span": {
        "byte_end": 132,
        "byte_start": 126,
        "col": 1,
        "line": 6
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “bogus” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 88,
        "byte_start": 81,
        "col": 1,
        "line": 4
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “bogus” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 88,
        "byte_start": 81,
        "col": 1,
        "line": 4
      }
    },
    {
      "category": "I18n",
      "code": "i18n.meta.charset.mismatch",
      "message": "Internal encoding declaration “windows-1251” disagrees with the actual encoding of the document (“utf-8”).",
      "severity": "Warning",
      "span": {
        "byte_end": 117,
        "byte_start": 88,
        "col": 8,
        "line": 4
      }
    }
  ],
  "source_name": "html/syntax/charset/after-bogus.html"
}
```
