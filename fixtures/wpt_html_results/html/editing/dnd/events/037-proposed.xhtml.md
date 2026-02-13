# html/editing/dnd/events/037-proposed.xhtml

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/events/037-proposed.xhtml",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<html xmlns="http://www.w3.org/1999/xhtml">
  <head>
    <title>Drag and drop without cancelling dragenter and without body</title>
    <style type="text/css">
html {
  padding:20px;
}
head + div {
  height: 100px;
  width: 100px;
  background: orange;
  display: inline-block;
}
head + div + div {
  height: 100px;
  width: 100px;
  margin-left: 100px;
  background: blue;
  display: inline-block;
}
    </style>
    <script type="text/javascript"><![CDATA[
//Drag passes from orange to root element. Dragenter fires at root element.
//Dragenter is not cancelled. Body does not exist. Dragenter fires at root element again.
//Drag passes from root element to blue. Dragenter fires at blue.
//Dragenter is not cancelled. Body does not exist. Current target element is root element.
//Drag passes from blue to root element. Current target element is already root element.
//Drag passes from root element to orange. Dragenter fires at orange, and is cancelled.
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
  var blue = document.getElementsByTagName('div')[1];
  blue.ondragenter = blue.ondragover = blue.ondragleave = function (e) {
    sequence[sequence.length] = 'blue.'+e.type;
  };
  document.documentElement.ondragenter = document.documentElement.ondragleave = function (e) {
    if( e.target != this ) { return; }
    sequence[sequence.length] = 'html.'+e.type;
  };
  document.documentElement.ondragover = function (e) {
    if( e.target != this ) { return; }
    if( sequence[sequence.length-1] != 'html.dragover' ) {
      sequence[sequence.length] = 'html.dragover';
    }
  };
  document.ondragenter = document.ondragleave = document.ondragover = document.ondragleave = function (e) {
    if( e.target != this ) { return; }
    sequence[sequence.length] = 'document.'+e.type;
  };
  orange.ondragend = function (e) {
    sequence = sequence.join('=>')
    var desiredsequence = (['orange.dragenter','orange.dragover','html.dragenter','html.dragenter','orange.dragleave','html.dragover','blue.dragenter','html.dragover','orange.dragenter','html.dragleave','orange.dragover','orange.drop']).join('=>')
    if( sequence == desiredsequence ) {
      document.getElementsByTagName('div')[2].textContent = 'PASS';
    } else {
      document.getElementsByTagName('div')[2].textContent = 'FAIL, got: '+sequence+' instead of: '+desiredsequence;
    }
  };
};
    ]]></script>
  </head>
  <!--body-->

    <div draggable="true"></div><div></div>
    <div>&#160;</div>
    <p>Drag the orange square onto the blue square, then back to the orange square, and release it.</p>
    <noscript><p>Enable JavaScript and reload</p></noscript>

  <!--/body-->
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
        "line": 4
      }
    },
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 449,
        "byte_start": 418,
        "col": 5,
        "line": 22
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
  "source_name": "html/editing/dnd/events/037-proposed.xhtml"
}
```
