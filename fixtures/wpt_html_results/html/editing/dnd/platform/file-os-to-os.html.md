# html/editing/dnd/platform/file-os-to-os.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/platform/file-os-to-os.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
  <head>
    <title>drag and drop - dragging items from the OS to the OS, via the browser window</title>
  </head>
  <body>

    <p>This test is only relevant on platforms where it is possible to switch applications in mid-drag (eg. alt+tab, dragging over taskbar buttons, dragging between restored windows).</p>
    <p>This testcase requires an external application that accepts dragging and dropping of files - eg. your system's file manager. Ensure that two application windows are open for the external application, showing different folders.</p>
    <p>Select a file in the first external application window. Drag the file over the browser window, then over the other external application window and release it. Pass if the file is copied/moved to the second window, as expected by the system.</p>

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
  "source_name": "html/editing/dnd/platform/file-os-to-os.html"
}
```
