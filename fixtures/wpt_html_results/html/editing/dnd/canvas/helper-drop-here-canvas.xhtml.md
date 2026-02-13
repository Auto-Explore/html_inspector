# html/editing/dnd/canvas/helper-drop-here-canvas.xhtml

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/canvas/helper-drop-here-canvas.xhtml",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<?xml version="1.0" encoding="utf-8"?>
<html xmlns="http://www.w3.org/1999/xhtml">
<head>
<title>Canvas drag and drop: helper file</title>
<script type="application/ecmascript">
function paint(color)
  {var canvas = document.querySelector('canvas'),
  c = canvas.getContext('2d');
  c.fillStyle = color;
  c.beginPath();
  c.moveTo(0,0);
  c.lineTo(100,0);
  c.lineTo(100,100);
  c.lineTo(0,100);
  c.closePath();
  c.fill();}
function start(event)
  {event.dataTransfer.effectAllowed = 'copy';
  event.dataTransfer.setData('text/plain', 'green');}
</script>
</head>
<body onload="paint('gray')">
<p>
  <canvas width="100" height="100" ondragenter="event.preventDefault()" ondragover="return false" ondrop="paint(event.dataTransfer.getData('text/plain'))">Canvas</canvas>
</p>
</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 177,
        "byte_start": 139,
        "col": 1,
        "line": 5
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
  "source_name": "html/editing/dnd/canvas/helper-drop-here-canvas.xhtml"
}
```
