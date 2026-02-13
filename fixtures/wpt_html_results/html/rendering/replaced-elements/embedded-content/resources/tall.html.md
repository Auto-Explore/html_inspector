# html/rendering/replaced-elements/embedded-content/resources/tall.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/replaced-elements/embedded-content/resources/tall.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<body style="background: blue">
  <div style="position: fixed; left: 0; top: 0; width: 100%; height: 100px; background: yellow"></div>
  <div style="position: fixed; left: 0; bottom: 0; width: 100%; height: 100px; background: green"></div>
  <div style="height: 2000px"></div>
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
  "source_name": "html/rendering/replaced-elements/embedded-content/resources/tall.html"
}
```
