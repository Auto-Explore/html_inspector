# html/editing/dnd/platform/drag-link-manual.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/platform/drag-link-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
  <head>
    <title>Dragging vs selecting links</title>
  </head>
  <body>

    <p><a href="">Test link, test link, test link, test link, test link, test link, test link, test link, test link</a></p>
    <p>Drag the test link above. When dragging vertically, it should drag the link. When dragging horizontally, it should select the text within the link.</p>

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
  "source_name": "html/editing/dnd/platform/drag-link-manual.html"
}
```
