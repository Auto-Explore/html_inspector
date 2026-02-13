# html/syntax/charset/document-write.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/syntax/charset/document-write.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<head>
<link rel="mismatch" href="references/document-write-ref.html">
<script>document.write('<meta char' + 'set="windows-1251">');</script>
</head>
<body>
<p>Meta from <code>document.write</code> (with concatenation in the middle of <code>charset</code> to require execution for effect).</p>
<p>Test: �</p>
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
  "source_name": "html/syntax/charset/document-write.html"
}
```
