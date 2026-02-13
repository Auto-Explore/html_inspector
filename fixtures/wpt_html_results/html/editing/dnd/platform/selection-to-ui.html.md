# html/editing/dnd/platform/selection-to-ui.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/platform/selection-to-ui.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
  <head>
    <title>Dragging text to UI widgets</title>
  </head>
  <body>

    <ol>
      <li>Select some text on this page.</li>
      <li>Drag the selection to your browser's address field. Fail if the mouse cursor shows that the text cannot be dropped.</li>
      <li>Release it. Pass if the selected text appears in the address field.</li>
      <li>Repeat the test with other UI text inputs.</li>
    </ol>

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
  "source_name": "html/editing/dnd/platform/selection-to-ui.html"
}
```
