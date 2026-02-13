# html/browsers/history/the-location-interface/resources/iframe-contents.sub.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/history/the-location-interface/resources/iframe-contents.sub.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<html>
  <head>
    <title>Resource file for test of Referer with location.replace</title>
  </head>
  <body>
    <div></div>
    <script>
      const referer = "{{header_or_default(referer, missing)}}"
      window.parent.postMessage(referer);
      document.querySelector("div").textContent = `Referer header: ${referer}`;
    </script>
  </body>
</html>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/browsers/history/the-location-interface/resources/iframe-contents.sub.html"
}
```
