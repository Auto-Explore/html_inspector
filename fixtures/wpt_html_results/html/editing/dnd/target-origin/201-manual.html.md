# html/editing/dnd/target-origin/201-manual.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/target-origin/201-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
  <head>
    <title>allowTargetOrigin with real dataTransfer should block dragenter, dragover, dragleave and drop synthetic events</title>
    <style type="text/css">
p + div { height: 100px; width: 100px; background: orange; }
    </style>
    <script type="text/javascript">
window.onload = function () {
  var orange = document.getElementsByTagName('div')[0], targ = document.getElementsByTagName('div')[1], evtdone = {}, fails = [];
  orange.ondragstart = function (e) {
    var evt;
    e.dataTransfer.effectAllowed = 'copy';
    e.dataTransfer.setData('text','dummy text');
    try {
      e.dataTransfer.allowTargetOrigin('http://example.com');
    } catch(e) {
      fails[fails.length] = 'allowTargetOrigin threw an error: '+e;
    }
    try {
      evt = new DragEvent('dragstart', {dataTransfer:e.dataTransfer});
      targ.dispatchEvent(evt);
      evt = new DragEvent('drag', {dataTransfer:e.dataTransfer});
      targ.dispatchEvent(evt);
      evt = new DragEvent('dragenter', {dataTransfer:e.dataTransfer});
      targ.dispatchEvent(evt);
      evt = new DragEvent('dragover', {dataTransfer:e.dataTransfer});
      targ.dispatchEvent(evt);
      evt = new DragEvent('dragleave', {dataTransfer:e.dataTransfer});
      targ.dispatchEvent(evt);
      evt = new DragEvent('drop', {dataTransfer:e.dataTransfer});
      targ.dispatchEvent(evt);
      evt = new DragEvent('dragend', {dataTransfer:e.dataTransfer});
      targ.dispatchEvent(evt);
    } catch(e) {
      fails[fails.length] = 'Synthetic event threw an error: '+e;
    }
    if( !evtdone.dragstart ) {
      fails[fails.length] = 'dragstart did not fire';
    }
    if( !evtdone.drag ) {
      fails[fails.length] = 'drag did not fire';
    }
    if( !evtdone.dragend ) {
      fails[fails.length] = 'dragend did not fire';
    }
    if( evtdone.dragenter ) {
      fails[fails.length] = 'dragenter fired';
    }
    if( evtdone.dragover ) {
      fails[fails.length] = 'dragover fired';
    }
    if( evtdone.dragleave ) {
      fails[fails.length] = 'dragleave fired';
    }
    if( evtdone.drop ) {
      fails[fails.length] = 'drop fired';
    }
    document.getElementsByTagName('p')[0].innerHTML = fails.length ? ( 'FAIL:<br>' + fails.join('<br>') ) : 'PASS';
  };
  targ.ondragstart = function (e) {
    evtdone[e.type] = true;
  };
  targ.ondragenter = targ.ondragover = targ.ondrop = function (e) {
    e.preventDefault();
    evtdone[e.type] = true;
  };
  targ.ondrag = targ.ondragleave = function (e) {
    evtdone[e.type] = true;
  };
  targ.ondragend = function (e) {
    evtdone[e.type] = true;
  };
};
    </script>
  </head>
  <body>
    <p>Drag the orange square to the right until the drag placeholder appears, then release it. Fail if this text does not change.</p>
    <div draggable="true"></div>
    <div></div>
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
        "byte_end": 189,
        "byte_start": 166,
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
        "byte_end": 299,
        "byte_start": 268,
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
  "source_name": "html/editing/dnd/target-origin/201-manual.html"
}
```
