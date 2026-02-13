# html/editing/dnd/platform/keyboardshortcuts.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/platform/keyboardshortcuts.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
  <head>
    <title>Keyboard shortcuts during drag/drop</title>
  </head>
  <body>

    <ol>
      <li>Select some text in this sentence and begin dragging it.</li>
      <li>While dragging, use your keyboard shortcut to reload the page. It should work without cancelling the drag.</li>
      <li>While dragging, use your keyboard shortcut to select all text on the page. It should work without cancelling the drag.</li>
      <li>While dragging, use your keyboard shortcut to open a new tab. It should work without cancelling the drag.</li>
      <li>While dragging, use your keyboard shortcut to switch to another tab. It should work without cancelling the drag.</li>
      <li>While dragging, use your keyboard shortcuts to go back and forward in history (<a href="#next">use this link first to add a history entry if needed</a>). It should work without cancelling the drag.</li>
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
  "source_name": "html/editing/dnd/platform/keyboardshortcuts.html"
}
```
