# html/editing/dnd/platform/text-to-os.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/platform/text-to-os.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
  <head>
    <title>drag and drop - dragging plain text onto the OS</title>
  </head>
  <body>

    <p>This test is only relevant on platforms where it is possible to switch applications in mid-drag (eg. alt+tab, dragging over taskbar buttons, dragging between restored windows).</p>
    <p>This testcase requires an external application that accepts dropping of text from other applications - eg. Wordpad (write.exe) on Windows. Ensure that the external application is open.</p>
    <p draggable="true">Drag this paragraph to the other application and release it. Pass if &quot;PASS&quot; appears in the other application, and the drag placeholder disappears when the drag is released.</p>
    <script type="text/javascript">
document.getElementsByTagName('p')[2].ondragstart = function (e) {
  e.dataTransfer.effectAllowed = 'copy';
  e.dataTransfer.setData('Text', 'PASS');
};
    </script>

  </body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 749,
        "byte_start": 718,
        "col": 5,
        "line": 11
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/editing/dnd/platform/text-to-os.html"
}
```
