# html/editing/dnd/platform/taskbarminimise.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/platform/taskbarminimise.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
  <head>
    <title>drag and drop - minimising using the taskbar</title>
  </head>
  <body>

    <p>This test is only relevant on platforms where dragging over the taskbar (or a specific button on it) will minimise all applications.</p>
    <p>Select this text. Drag the selection downwards, over a blank part of the system taskbar (or a minimise-all button if provided by the system). Hold the drag until all applications have minimised, then drag upwards over the desktop. Pass if the drag placeholder is still visible.</p>

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
  "source_name": "html/editing/dnd/platform/taskbarminimise.html"
}
```
