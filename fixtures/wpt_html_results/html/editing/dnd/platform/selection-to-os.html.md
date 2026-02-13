# html/editing/dnd/platform/selection-to-os.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/platform/selection-to-os.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
  <head>
    <title>drag and drop - dragging a selection onto the OS</title>
  </head>
  <body>

    <p>This test is only relevant on platforms where it is possible to switch applications in mid-drag (eg. alt+tab, dragging over taskbar buttons, dragging between restored windows).</p>
    <p>This testcase requires an external application that accepts dropping of text from other applications - eg. Wordpad (write.exe) on Windows. Ensure that the external application is open.</p>
    <p>Select this text. Drag the selection to the other application and release it. Pass if the text you selected appears in the other application, and the drag placeholder disappears when the drag is released.</p>
    <p>Disable JavaScript and repeat the test. It should still pass.</p>

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
  "source_name": "html/editing/dnd/platform/selection-to-os.html"
}
```
