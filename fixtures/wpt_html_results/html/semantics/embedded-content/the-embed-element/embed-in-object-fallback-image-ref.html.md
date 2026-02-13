# html/semantics/embedded-content/the-embed-element/embed-in-object-fallback-image-ref.html

Counts:
- errors: 2
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-embed-element/embed-in-object-fallback-image-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Embed Reftest Reference</title>
<link rel="author" title="Peng Zhou" href="mailto:zhoupeng.1996@bytedance.com">
<style>
  embed {
    background-color: red;
    height: 100px;
    width: 100px;
  }
</style>
<body>
  <p>Test passes if there is <strong>red</strong>.</p>
  <embed src="/images/red-16x16.png" type="image/png"></embed>
</body>
</body>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “embed”.",
      "severity": "Error",
      "span": {
        "byte_end": 377,
        "byte_start": 369,
        "col": 55,
        "line": 14
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “body”.",
      "severity": "Error",
      "span": {
        "byte_end": 393,
        "byte_start": 386,
        "col": 1,
        "line": 16
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
  "source_name": "html/semantics/embedded-content/the-embed-element/embed-in-object-fallback-image-ref.html"
}
```
