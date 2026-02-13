# html/editing/dnd/platform/selection-from-os.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/platform/selection-from-os.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
  <head>
    <title>drag and drop - dragging a selection from the OS</title>
  </head>
  <body>

    <p>This test is only relevant on platforms where it is possible to switch applications in mid-drag (eg. alt+tab, dragging over taskbar buttons, dragging between restored windows).</p>
    <p>This testcase requires an external application that allows dragging of selections into other applications - eg. Wordpad (write.exe) on Windows. Ensure that the external application is open.</p>
    <p>Move the browser window so it sits about 200 pixels down from the top of the screen.</p>
    <p>Subtest 1. Write some text into the external application (if needed), containing both unicode and Latin characters. Select the text in the external application, and drag the selection into the following input:<br><textarea rows="3" cols="50"></textarea><br>Pass if the text you selected appears in the input.</p>
    <p>Subtest 2. Select the text in the external application, and drag the selection into the following block:
    <span style="background:orange;display:block;min-height:100px;width:300px;" ondragenter="return false;" ondragover="return false;" ondrop="this.innerHTML = arguments[0].dataTransfer.getData('text/plain');return false;"></span>
    Pass if the text you selected appears in the block.</p>

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
  "source_name": "html/editing/dnd/platform/selection-from-os.html"
}
```
