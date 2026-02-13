# html/editing/dnd/platform/selection-between-ui.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/platform/selection-between-ui.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
  <head>
    <title>Dragging text from UI widgets to each other</title>
  </head>
  <body>

    <ol>
      <li>Select some text in the address bar.</li>
      <li>Drag the selection to another text input in the UI, and release it.</li>
      <li>Pass if:<ol>
        <li>A visible representation of the selected text appears to be dragged.</li>
        <li>The mouse cursor shows that the drop will be allowed over the input.</li>
        <li>The selected text appears in the input.</li>
      </ol></li>
      <li>Repeat the test with other UI text inputs as drag source/destination.</li>
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
  "source_name": "html/editing/dnd/platform/selection-between-ui.html"
}
```
