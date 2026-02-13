# html/editing/dnd/platform/html-unicode-to-os.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/platform/html-unicode-to-os.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
  <head>
    <title>drag and drop - dragging HTML onto the OS</title>
  </head>
  <body>

    <p>This test is only relevant on platforms where it is possible to switch applications in mid-drag (eg. alt+tab, dragging over taskbar buttons, dragging between restored windows).</p>
    <p>This testcase requires an external application that accepts dropping of unicode HTML from other applications - eg. Google Chrome (not Firefox or Internet Explorer). Load <a href="html-to-os-HELPER-FILE.html">the helper file</a> in the external application.</p>
    <p draggable="true">Drag this paragraph to the other application and release it. De-select the text in that application if it is selected. Pass if &quot;Pass if this text is on a green background 中文אידישрусский&quot; appears in the other application, and if it has a green background, and if the drag placeholder disappears when the drag is released.</p>
    <script type="text/javascript">
document.getElementsByTagName('p')[2].ondragstart = function (e) {
  e.dataTransfer.effectAllowed = 'copy';
  e.dataTransfer.setData('text/html', '<span style="background:lime;color:black;">Pass if this text is on a green background 中文אידישрусский</span>');
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
        "byte_end": 979,
        "byte_start": 948,
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
  "source_name": "html/editing/dnd/platform/html-unicode-to-os.html"
}
```
