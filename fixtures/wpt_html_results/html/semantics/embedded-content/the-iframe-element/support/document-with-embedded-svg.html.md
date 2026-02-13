# html/semantics/embedded-content/the-iframe-element/support/document-with-embedded-svg.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/support/document-with-embedded-svg.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<body>
<script>
["embed", "frame", "iframe", "object"].forEach(name => {
  const frame = document.body.appendChild(document.createElement(name));
  const attr = name !== "object" ? "src" : "data";
  frame[attr] = "svg.svg";
});
</script>
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
  "source_name": "html/semantics/embedded-content/the-iframe-element/support/document-with-embedded-svg.html"
}
```
