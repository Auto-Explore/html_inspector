# html/syntax/charset/references/after-head-in-1kb-ref.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/syntax/charset/references/after-head-in-1kb-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<head>
<meta charset="utf-8">
</head>
<body>
<p>After <code>head</code>, before <code>body</code>, within first kilobyte.</p>
<p>Test: ж</p>
<p>If &#x0436;, meta takes effect</p>
</body>
```

```json
{
  "messages": [
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
  "source_name": "html/syntax/charset/references/after-head-in-1kb-ref.html"
}
```
