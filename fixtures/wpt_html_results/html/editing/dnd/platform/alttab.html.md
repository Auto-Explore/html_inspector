# html/editing/dnd/platform/alttab.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/platform/alttab.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
  <head>
    <title>drag and drop - alt+tab while dragging</title>
  </head>
  <body>

    <p>This test is only relevant on platforms where alt+tab (or some equivalent) switches applications.</p>
    <p>Ensure that at least one other application is open. Select this text. Drag the selection downwards a little, then alt+tab (or your system's equivalent) to the other application. Pass if the drag placeholder continues to follow the mouse/pointing device. Release the drag. Pass if the drag placeholder disappears.</p>

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
  "source_name": "html/editing/dnd/platform/alttab.html"
}
```
