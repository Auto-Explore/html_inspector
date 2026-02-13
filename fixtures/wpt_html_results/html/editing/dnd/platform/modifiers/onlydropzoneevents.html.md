# html/editing/dnd/platform/modifiers/onlydropzoneevents.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/platform/modifiers/onlydropzoneevents.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
  <head>
    <title>Modifier keys being used with a dropzone attribute and dragenter/dragover events</title>
    <style type="text/css">
div:first-child {
  height: 100px;
  width: 100px;
  background: orange;
  display: inline-block;
}
div:first-child + div {
  height: 100px;
  width: 100px;
  background: blue;
  display: inline-block;
}
div:first-child + div + div {
  height: 100px;
  width: 100px;
  background: fuchsia;
  display: inline-block;
}
table {
  display: inline-table;
  margin-right: 1em;
  border-collapse: collapse;
}
table, th, td {
  border: 1px solid black;
}
thead th {
  background: silver;
  color: black;
}
    </style>
    <script type="text/javascript">
window.onload = function () {
  var orange = document.getElementsByTagName('div')[0];
  orange.ondragstart = function (e) {
    e.dataTransfer.setData('text/plain','http://example.com/');
    e.dataTransfer.effectAllowed = 'copy';
  };
  var fuchsia = document.getElementsByTagName('div')[2], fde, fdo;
  fuchsia.ondragenter = function (e) {
    fde = e.dataTransfer.dropEffect;
  };
  fuchsia.ondragover = function (e) {
    fdo = e.dataTransfer.dropEffect;
  };
  fuchsia.ondrop = function (e) {
    //dropzone overrides the modifier, always, and ignores effectAllowed
    e.preventDefault();
    var sequence = ([fde,fdo,e.dataTransfer.dropEffect]).join('=&gt;')
    var desiredsequence = (['move','move','link']).join('=&gt;')
    if( sequence == desiredsequence ) {
      document.getElementsByTagName('div')[3].innerHTML = 'PASS';
    } else {
      document.getElementsByTagName('div')[3].innerHTML = 'FAIL, got:<br>'+sequence+'<br>instead of:<br>'+desiredsequence;
    }
  };
};
    </script>
  </head>
  <body>

    <div draggable="true"></div>
    <div></div>
    <div dropzone="link string:text/plain"></div>
    <div>&nbsp;</div>
    <ol>
      <li>Drag the orange square over the blue square</li>
      <li>Press the relevant modifier keys for your platform to request a &quot;move&quot; drop effect (eg. Shift on Windows/Unix/Linux, Command on Mac)</li>
      <li>Continue dragging over the pink square</li>
      <li>If supported by the platform, the mouse cursor should show that a &quot;link&quot; drop effect will be used</li>
      <li>Release the drag, then the keys</li>
      <li>Fail if no new text appears above this list</li>
    </ol>
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
        "byte_end": 159,
        "byte_start": 136,
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
        "byte_end": 706,
        "byte_start": 675,
        "col": 5,
        "line": 37
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
  "source_name": "html/editing/dnd/platform/modifiers/onlydropzoneevents.html"
}
```
