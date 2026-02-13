# html/syntax/charset/non-ascii-in-title-before.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/syntax/charset/non-ascii-in-title-before.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<head>
<link rel="match" href="references/non-ascii-in-title-before-ref.html">
<title>�</title>
<meta charset="windows-1251">
</head>
<body>
<p>Normal meta. Non-ASCII in title before.</p>
<p>Test: �</p>
<p>If &#x0436;, meta takes effect</p>
</body>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.meta.charset.mismatch",
      "message": "Internal encoding declaration “windows-1251” disagrees with the actual encoding of the document (“utf-8”).",
      "severity": "Warning",
      "span": {
        "byte_end": 141,
        "byte_start": 112,
        "col": 1,
        "line": 5
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
  "source_name": "html/syntax/charset/non-ascii-in-title-before.html"
}
```
