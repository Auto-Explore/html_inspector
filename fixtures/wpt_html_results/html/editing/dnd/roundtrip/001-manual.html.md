# html/editing/dnd/roundtrip/001-manual.html

Counts:
- errors: 1
- warnings: 5
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/roundtrip/001-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<?xml version="1.0" encoding="utf-8"?>
<html xmlns="http://www.w3.org/1999/xhtml">
<head>
<title>Canvas drag and drop roundtrip</title>
<style type="text/css">
img
  {margin:0 2px;}
</style>
<script type="application/ecmascript">
function addImage(event)
  {var c = document.createElement('img');
  c.setAttribute('src',event.dataTransfer.getData('text/uri-list').replace(/\r\n$/,''));
  document.querySelector('p').appendChild(c);}
function start(event)
  {event.dataTransfer.effectAllowed = 'copy';
  event.dataTransfer.setData('text/uri-list', document.querySelector('canvas').toDataURL('image/png'));}
</script>
</head>
<body>
<p>
  <canvas width="100" height="100" draggable="true" ondragstart="start(event)" ondragenter="event.preventDefault()" ondragover="return false" ondrop="addImage(event)">Canvas</canvas>
</p>
<p>Drag canvas pattern outside browser window and then drag it back and drop on itself. It should be duplicated once you drop it.</p>
<script type="application/ecmascript">
var canvas = document.querySelector('canvas'),
c = canvas.getContext('2d');
for(var x = 0; x != 50; x++)
  {c.fillStyle = (x%2 == 0)?'navy':'white';
  c.beginPath();
  c.moveTo(x,x);
  c.lineTo(100-x,x);
  c.lineTo(100-x,100-x);
  c.lineTo(x,100-x);
  c.closePath();
  c.fill();}
</script>
</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.tokenizer.processing_instruction",
      "message": "Saw “<?”. Probable cause: Attempt to use an XML processing instruction in HTML. (XML processing instructions are not supported in HTML.)",
      "severity": "Warning",
      "span": {
        "byte_end": 2,
        "byte_start": 0,
        "col": 1,
        "line": 1
      }
    },
    {
      "category": "Html",
      "code": "html.parser.start_tag_without_doctype",
      "message": "Start tag seen without seeing a doctype first. Expected “<!DOCTYPE html>”.",
      "severity": "Error",
      "span": {
        "byte_end": 82,
        "byte_start": 39,
        "col": 1,
        "line": 2
      }
    },
    {
      "category": "Html",
      "code": "html.style.type.unnecessary",
      "message": "The “type” attribute for the “style” element is not needed and should be omitted.",
      "severity": "Warning",
      "span": {
        "byte_end": 159,
        "byte_start": 136,
        "col": 1,
        "line": 5
      }
    },
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 229,
        "byte_start": 191,
        "col": 1,
        "line": 9
      }
    },
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 995,
        "byte_start": 957,
        "col": 1,
        "line": 24
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
  "source_name": "html/editing/dnd/roundtrip/001-manual.html"
}
```
