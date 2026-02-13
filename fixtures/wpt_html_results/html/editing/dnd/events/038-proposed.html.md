# html/editing/dnd/events/038-proposed.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/events/038-proposed.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
  <head>
    <title>Drag and drop without cancelling dragenter and without body or html</title>
    <style type="text/css">
body > div:first-child {
  height: 100px;
  width: 100px;
  background: orange;
  display: inline-block;
}
iframe {
  height: 100px;
  width: 100px;
  margin-left: 100px;
  display: inline-block;
  border: none;
}
    </style>
    <script type="text/javascript">
//Drag passes from parent to blue. Dragenter fires at blue. Root element is deleted.
//Dragenter is not cancelled. Body does not exist. Root element does not exist.
//Current target element is set to null. Drag now points at unrendered document - null.
//Current target element remains null.
//Drag passes over parent to orange. Dragenter fires at orange, and is cancelled.
window.onload = function () {
  var orange = document.getElementsByTagName('div')[0], sequence = [];
  orange.ondragstart = function (e) {
    e.dataTransfer.setData('text','hello');
    e.dataTransfer.effectAllowed = 'copy';
  };
  orange.ondragenter = orange.ondrop = function (e) {
    sequence[sequence.length] = 'orange.'+e.type;
    e.preventDefault();
  };
  orange.ondragleave = function (e) {
    sequence[sequence.length] = 'orange.dragleave';
  };
  orange.ondragover = function (e) {
    if( sequence[sequence.length-1] != 'orange.dragover' ) {
      sequence[sequence.length] = 'orange.dragover';
    }
    e.preventDefault();
  };
  var blue = document.getElementsByTagName('iframe')[0].contentDocument;
  if( !blue.documentElement ) { blue.appendChild(blue.createElement('html')); }
  blue.documentElement.style.margin = '0';
  blue.documentElement.style.padding = '0';
  if( !blue.body ) { blue.documentElement.appendChild(blue.createElement('body')); }
  blue.body.style.margin = '0';
  blue.body.style.padding = '0';
  var bluediv = blue.body.appendChild(blue.createElement('div'));
  bluediv.style.height = '100px';
  bluediv.style.width = '100px';
  bluediv.style.background = 'blue';
  bluediv.ondragenter = bluediv.ondragover = function (e) {
    sequence[sequence.length] = 'blue.'+e.type;
    if( blue.documentElement ) { blue.removeChild(blue.documentElement); }
  };
  blue.ondragenter = blue.ondragover = blue.ondragleave = function (e) {
    if( e.target != this ) { return; }
    sequence[sequence.length] = 'bluedocument.'+e.type;
  };
  orange.ondragend = function (e) {
    sequence = sequence.join('=&gt;')
    var desiredsequence = (['orange.dragenter','orange.dragover','orange.dragleave','blue.dragenter','orange.dragenter','orange.dragover','orange.drop']).join('=&gt;')
    if( sequence == desiredsequence ) {
      document.getElementsByTagName('div')[1].innerHTML = 'PASS';
    } else {
      document.getElementsByTagName('div')[1].innerHTML = 'FAIL, got:<br>'+sequence+'<br>instead of:<br>'+desiredsequence;
    }
  };
};
    </script>
  </head>
  <body>

    <div draggable="true"></div><iframe src="about:blank"></iframe>
    <div>&nbsp;</div>
    <p>Drag the orange square onto the blue square, then back to the orange square, and release it.</p>
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
        "byte_end": 146,
        "byte_start": 123,
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
        "byte_end": 409,
        "byte_start": 378,
        "col": 5,
        "line": 20
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
  "source_name": "html/editing/dnd/events/038-proposed.html"
}
```
