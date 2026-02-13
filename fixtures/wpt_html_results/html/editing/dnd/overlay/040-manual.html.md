# html/editing/dnd/overlay/040-manual.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/overlay/040-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<?xml version="1.0" encoding="utf-8"?>
<html xmlns="http://www.w3.org/1999/xhtml">
<head>
<title>Cursor position and drag image</title>
<script type="application/ecmascript">
function start(event)
  {event.dataTransfer.effectAllowed = 'copy';
  event.dataTransfer.setDragImage(document.querySelector('canvas'), 50, 50);}
</script>
</head>
<body>
<p><a href="data:text/plain,1" ondragstart="start(event)">Drag me</a></p>
<p>Try to drag link above. Feedback overlay should be based on canvas below and mouse pointer should be anchored in its center.</p>
<p>
  <canvas width="100" height="100">Canvas</canvas>
</p>
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
        "byte_end": 18,
        "byte_start": 16,
        "col": 1,
        "line": 2
      }
    },
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 190,
        "byte_start": 152,
        "col": 1,
        "line": 6
      }
    },
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 666,
        "byte_start": 628,
        "col": 1,
        "line": 18
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
  "source_name": "html/editing/dnd/overlay/040-manual.html"
}
```
