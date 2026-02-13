# html/editing/dnd/platform/selection-ui-to-self.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/platform/selection-ui-to-self.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
  <head>
    <title>Dragging text from UI widgets to themselves</title>
  </head>
  <body>

    <ol>
      <li>Select all text in the address bar.</li>
      <li>Drag the selection around a little, and release it over the address bar again.</li>
      <li>Try to select the text in this sentence.</li>
      <li>Pass if:<ol>
        <li>A visible representation of the selected text appears to be dragged.</li>
        <li>The mouse cursor shows that the drop will be allowed over the address bar.</li>
        <li>The text on this page can be selected afterwards.</li>
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
  "source_name": "html/editing/dnd/platform/selection-ui-to-self.html"
}
```
