# html/syntax/charset/in-object.html

Counts:
- errors: 3
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/syntax/charset/in-object.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<head>
<link rel="match" href="references/in-object-ref.html">
<object><meta charset="windows-1251"></object>
</head>
<body>
<p>Meta in <code>object</code>.</p>
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
        "byte_end": 133,
        "byte_start": 126,
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
        "byte_end": 140,
        "byte_start": 134,
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
        "byte_end": 140,
        "byte_start": 134,
        "col": 1,
        "line": 6
      }
    },
    {
      "category": "Html",
      "code": "html.object.data.missing",
      "message": "Element “object” is missing required attribute “data”.",
      "severity": "Warning",
      "span": {
        "byte_end": 87,
        "byte_start": 79,
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
        "byte_end": 116,
        "byte_start": 87,
        "col": 9,
        "line": 4
      }
    }
  ],
  "source_name": "html/syntax/charset/in-object.html"
}
```
