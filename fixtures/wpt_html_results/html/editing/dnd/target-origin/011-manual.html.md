# html/editing/dnd/target-origin/011-manual.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/target-origin/011-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
  <head>
    <title>allowTargetOrigin should only block dragenter, dragover, dragleave and drop events</title>
    <style type="text/css">
div { height: 100px; width: 100px; background: orange; margin: 0; padding: 0; float: left; }
div + div { background: blue; }
    </style>
    <script type="text/javascript">
window.onload = function () {
  var orange = document.getElementsByTagName('div')[0], evtdone = {}, fails = [];
  orange.ondragstart = function (e) {
    evtdone[e.type] = true;
    e.dataTransfer.effectAllowed = 'copy';
    e.dataTransfer.setData('text','dummy text');
    try {
      e.dataTransfer.allowTargetOrigin('http://example.com');
    } catch(e) {
      fails[fails.length] = 'allowTargetOrigin threw an error: '+e;
    }
  };
  orange.ondragenter = orange.ondragover = orange.ondrop = function (e) {
    e.preventDefault();
    evtdone[e.type] = true;
  };
  orange.ondrag = orange.ondragleave = function (e) {
    evtdone[e.type] = true;
  };
  orange.ondragend = function (e) {
    evtdone[e.type] = true;
    if( !evtdone.dragstart ) {
      fails[fails.length] = 'dragstart did not fire - how did that happen?';
    }
    if( !evtdone.drag ) {
      fails[fails.length] = 'drag did not fire';
    }
    if( !evtdone.dragend ) {
      fails[fails.length] = 'dragend did not fire - OK, who broke the testcase?';
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
};
    </script>
  </head>
  <body>
    <p>Drag the orange square over the blue square then back to the orange square, then release it. Fail if this text does not change.</p>
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
        "byte_end": 161,
        "byte_start": 138,
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
        "byte_end": 335,
        "byte_start": 304,
        "col": 5,
        "line": 9
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
  "source_name": "html/editing/dnd/target-origin/011-manual.html"
}
```
