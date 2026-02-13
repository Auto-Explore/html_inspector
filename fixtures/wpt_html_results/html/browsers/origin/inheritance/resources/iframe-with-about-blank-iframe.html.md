# html/browsers/origin/inheritance/resources/iframe-with-about-blank-iframe.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/origin/inheritance/resources/iframe-with-about-blank-iframe.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
  <body>
    <iframe src="about:blank"></iframe>
    <iframe src=""></iframe>
  </body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.iframe.src.empty",
      "message": "Bad value “” for attribute “src” on element “iframe”.",
      "severity": "Warning",
      "span": {
        "byte_end": 91,
        "byte_start": 76,
        "col": 5,
        "line": 5
      }
    },
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
  "source_name": "html/browsers/origin/inheritance/resources/iframe-with-about-blank-iframe.html"
}
```
