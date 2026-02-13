# html/editing/dnd/platform/modifiers/scriptmodified.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/platform/modifiers/scriptmodified.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
  <head>
    <title>Modifier keys selecting dropEffect with script overriding it</title>
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
  var dragenterbefore = '', dragenterafter = '', dragoverbefore = '', dragoverafter = '';
  var orange = document.getElementsByTagName('div')[0];
  orange.ondragstart = function (e) {
    e.dataTransfer.setData('text','http://example.com/');
    e.dataTransfer.effectAllowed = 'all';
  };
  var fuchsia = document.getElementsByTagName('div')[2];
  fuchsia.ondragenter = function (e) {
    e.preventDefault();
    if( e.dataTransfer.dropEffect != 'link' ) {
      dragenterbefore = e.dataTransfer.dropEffect;
    }
    try {
      e.dataTransfer.dropEffect = 'move';
    } catch(e) {}
    if( e.dataTransfer.dropEffect != 'move' ) {
      dragenterafter = e.dataTransfer.dropEffect;
    }
  };
  fuchsia.ondragover = function (e) {
    e.preventDefault();
    if( e.dataTransfer.dropEffect != 'link' ) {
      dragoverbefore = e.dataTransfer.dropEffect;
    }
    try {
      e.dataTransfer.dropEffect = 'move';
    } catch(e) {}
    if( e.dataTransfer.dropEffect != 'move' ) {
      dragoverafter = e.dataTransfer.dropEffect;
    }
  };
  fuchsia.ondrop = function (e) {
    e.preventDefault();
    document.getElementsByTagName('div')[3].innerHTML = ( dragenterbefore || dragenterafter || dragoverbefore || dragoverafter || e.dataTransfer.dropEffect != 'move' ) ? ( 'FAIL' +
      ( dragenterbefore ? ( '<br>dragenter.dropEffect was '+dragenterbefore+' instead of link' ) : '' ) +
      ( dragenterafter ? ( '<br>dragenter.dropEffect after writing was '+dragenterafter+' instead of move' ) : '' ) +
      ( dragoverbefore ? ( '<br>dragover.dropEffect was '+dragoverbefore+' instead of link' ) : '' ) +
      ( dragoverafter ? ( '<br>dragover.dropEffect after writing was '+dragoverafter+' instead of move' ) : '' ) +
      ( ( e.dataTransfer.dropEffect != 'move' ) ? ( '<br>drop.dropEffect was '+e.dataTransfer.dropEffect+' instead of move' ) : '' )
      ) : 'PASS';
  };
};
    </script>
  </head>
  <body>

    <div draggable="true"></div>
    <div></div>
    <div></div>
    <div>&nbsp;</div>
    <ol>
      <li>Drag the orange square over the blue square</li>
      <li>Press the relevant modifier keys for your platform to request a &quot;link&quot; drop effect (eg. Alt on Windows, Ctrl+Shift on Unix/Linux, Command+Option on Mac)</li>
      <li>Continue dragging over the pink square</li>
      <li>If supported by the platform, the mouse cursor should show that a &quot;move&quot; drop effect will be used</li>
      <li>Release the drag, then the keys</li>
      <li>Fail if no new text appears above this list</li>
    </ol>

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
        "byte_end": 139,
        "byte_start": 116,
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
        "byte_end": 686,
        "byte_start": 655,
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
  "source_name": "html/editing/dnd/platform/modifiers/scriptmodified.html"
}
```
