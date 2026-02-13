# html/editing/dnd/platform/selection-from-ui.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/platform/selection-from-ui.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
  <head>
    <title>Dragging text from UI widgets</title>
  </head>
  <body>

    <ol>
      <li>Select some text in the address bar.</li>
      <li>Drag the selection to the following textarea, and release it: <br><textarea rows="3" cols="50"></textarea></li>
      <li>Pass if:<ol>
        <li>A visible representation of the selected text appears to be dragged.</li>
        <li>The mouse cursor shows that the drop will be allowed over the textarea.</li>
        <li>The selected text appears in the textarea.</li>
      </ol></li>
      <li>Repeat the test with other UI text inputs, including ones that allow linebreaks (if any).</li>
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
  "source_name": "html/editing/dnd/platform/selection-from-ui.html"
}
```
