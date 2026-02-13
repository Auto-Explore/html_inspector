# html/editing/dnd/platform/file-to-system.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/platform/file-to-system.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
  <head>
    <title>Dragging a file to the system</title>
    <style type="text/css">
span { display: inline-block; height: 100px; width: 100px; background: orange; }
    </style>
    <script type="text/javascript">
window.onload = function () {
  var drag = document.getElementsByTagName('span')[0];
  drag.ondragstart = function (e) {
    e.dataTransfer.setData('text','PASS');
    e.dataTransfer.effectAllowed = 'copy';
    var filein = document.getElementsByTagName('input')[0];
    if( !filein.files ) {
      document.getElementsByTagName('p')[0].innerHTML = 'FAIL - file API is not supported.';
      return;
    }
    if( !filein.files[0] ) {
      document.getElementsByTagName('p')[0].innerHTML = 'FAIL - no file was found in the file input.';
      return;
    }
    var thefile = filein.files[0];
    try {
      e.dataTransfer.items.add(thefile);
    } catch(err) {
      document.getElementsByTagName('p')[0].innerHTML = 'FAIL - error when adding file';
      e.preventDefault();
      return;
    }
    if( e.dataTransfer.files.length != 1 ) {
      document.getElementsByTagName('p')[0].innerHTML = 'FAIL - file was not attached to data store';
      e.preventDefault();
    }
  };
};
    </script>
  </head>
  <body>
    <div>This test only applies to platforms where dropping a file onto a folder in the system's file manager copies/moves the file to that folder.</div>
    <ol>
      <li>Open an empty folder in your system's file manager.</li>
      <li>Select a non-empty file on your computer using the following input: <input type="file"></li>
      <li>Drag the orange square onto the folder in your system's file manager, and release it:<br><span draggable="true"></span></li>
      <li>Pass if the file is copied to the folder.</li>
    </ol>
    <p></p>
    <noscript><p>Enable JavaScript and reload</p></noscript>
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
        "byte_end": 108,
        "byte_start": 85,
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
        "byte_end": 238,
        "byte_start": 207,
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
  "source_name": "html/editing/dnd/platform/file-to-system.html"
}
```
