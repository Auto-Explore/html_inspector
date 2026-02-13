# html/semantics/embedded-content/the-iframe-element/iframe-loading-lazy-in-viewport-001.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/iframe-loading-lazy-in-viewport-001.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html class="reftest-wait">
<head>
  <meta charset="utf-8">
  <link rel="match" href="iframe-loading-lazy-in-viewport-ref.html">
  <link rel="help" href="https://bugzilla.mozilla.org/show_bug.cgi?id=1860041">
</head>
<body>
  <iframe loading="lazy" src="data:text/html,PASS" onload="document.documentElement.className = ''"></iframe>
  <script>
    document.querySelector("iframe").getBoundingClientRect();
  </script>
</body>
</html>
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
  "source_name": "html/semantics/embedded-content/the-iframe-element/iframe-loading-lazy-in-viewport-001.html"
}
```
