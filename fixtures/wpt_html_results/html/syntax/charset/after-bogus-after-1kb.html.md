# html/syntax/charset/after-bogus-after-1kb.html

Counts:
- errors: 3
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/syntax/charset/after-bogus-after-1kb.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<head>
<link rel="mismatch" href="references/after-bogus-after-1kb-ref.html">



























































































































































































































































































































































































































































































































































































































































































































































































































































































































































<bogus><meta charset="windows-1251">
</head>
<body>
<p>After <code>bogus</code>, before <code>head</code> end tag, after first kilobyte.</p>
<p>Test: �</p>
<p>If &#x0436;, meta takes effect</p>
</body>
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
        "byte_end": 1053,
        "byte_start": 1024,
        "col": 8,
        "line": 927
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “head”.",
      "severity": "Error",
      "span": {
        "byte_end": 1061,
        "byte_start": 1054,
        "col": 1,
        "line": 928
      }
    },
    {
      "category": "Html",
      "code": "html.parser.body.already_open",
      "message": "Start tag “body” seen but an element of the same type was already open.",
      "severity": "Error",
      "span": {
        "byte_end": 1068,
        "byte_start": 1062,
        "col": 1,
        "line": 929
      }
    },
    {
      "category": "Html",
      "code": "html.parser.cannot_recover",
      "message": "Cannot recover after last error. Any further errors will be ignored.",
      "severity": "Error",
      "span": {
        "byte_end": 1068,
        "byte_start": 1062,
        "col": 1,
        "line": 929
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “bogus” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1024,
        "byte_start": 1017,
        "col": 1,
        "line": 927
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “bogus” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 1024,
        "byte_start": 1017,
        "col": 1,
        "line": 927
      }
    },
    {
      "category": "I18n",
      "code": "i18n.meta.charset.mismatch",
      "message": "Internal encoding declaration “windows-1251” disagrees with the actual encoding of the document (“utf-8”).",
      "severity": "Warning",
      "span": {
        "byte_end": 1053,
        "byte_start": 1024,
        "col": 8,
        "line": 927
      }
    }
  ],
  "source_name": "html/syntax/charset/after-bogus-after-1kb.html"
}
```
