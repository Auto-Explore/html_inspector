# html/editing/dnd/target-origin/010-manual.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/target-origin/010-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
  <head>
    <title>A URL should prevent dropping on external applications</title>
    <style type="text/css">
div { height: 100px; width: 100px; background: orange; }
    </style>
    <script type="text/javascript">
window.onload = function () {
  document.getElementsByTagName("div")[0].ondragstart = function (e) {
    e.dataTransfer.effectAllowed = "copy";
    e.dataTransfer.setData("text","FAIL");
    e.dataTransfer.allowTargetOrigin("http://foo");
  };
};
    </script>
  </head>
  <body>
    <p>This test is only relevant on platforms where it is possible to switch applications in mid-drag (eg. alt+tab, dragging over taskbar buttons, dragging between restored windows).</p>
    <p>This testcase requires an external application that accepts dropping of text from other applications - eg. Wordpad (write.exe) on Windows. Ensure that the external application is open.</p>
    <p>Drag the orange block to the other application and release it. Fail if the word &quot;FAIL&quot; appears in the other application.</p>
    <noscript><p>Enable JavaScript and reload</p></noscript>
    <div draggable="true"></div>
  </body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.style.type.unnecessary",
      "message": "The “type” attribute for the “style” element is not needed and should be omitted.",
      "severity": "Warning",
      "span": {
        "byte_end": 133,
        "byte_start": 110,
        "col": 5,
        "line": 5
      }
    },
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 239,
        "byte_start": 208,
        "col": 5,
        "line": 8
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
  "source_name": "html/editing/dnd/target-origin/010-manual.html"
}
```
