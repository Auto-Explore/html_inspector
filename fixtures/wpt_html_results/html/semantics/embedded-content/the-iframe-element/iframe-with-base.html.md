# html/semantics/embedded-content/the-iframe-element/iframe-with-base.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/iframe-with-base.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
<head>
    <title>iframe With Base Tag</title>
    <link rel="match" href="iframe-with-base-ref.html">
    <base href="support/">
</head>
<body>
    <iframe src="blank.htm">
</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.base.must_come_before_link_or_script",
      "message": "The “base” element must come before any “link” or “script” elements in the document.",
      "severity": "Warning",
      "span": {
        "byte_end": 152,
        "byte_start": 130,
        "col": 5,
        "line": 6
      }
    },
    {
      "category": "Html",
      "code": "html.iframe.text.disallowed",
      "message": "Text not allowed in “iframe” in this context.",
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
  "source_name": "html/semantics/embedded-content/the-iframe-element/iframe-with-base.html"
}
```
