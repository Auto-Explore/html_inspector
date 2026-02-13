# html/editing/dnd/events/helper-drag-me-p-with-circle.xhtml

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/events/helper-drag-me-p-with-circle.xhtml",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<?xml version="1.0" encoding="utf-8"?>
<html xmlns="http://www.w3.org/1999/xhtml">
<head>
<title>Cross frame drag and drop: helper file</title>
<style type="text/css">
div
  {width:0;
  height:0;
  border:solid 50px silver;
  border-radius:50px;
  margin-left:auto;}
</style>
<script type="application/ecmascript">
var step = 1;
function start(event)
  {if(step++ == 1)
    {setColor('green silver silver silver');}
  else
    {step = 0;
    setColor('maroon');
    say('Dragstart should be first event to fire.')}
  }
function leavePage(event)
  {if(step++ > 1)
    {setColor('green green silver silver')}
  else
    {step = 0;
    setColor('maroon');
    say('Dragleave should fire after dragstart.')}
  }
function endDrag(event)
  {if(step++ > 2)
    {setColor('green')}
  else
    {step = 0;
    setColor('maroon');
    say('Dragend should fire after dragstart and dragleave.')}
  }
function say(it)
  {document.querySelector('pre').appendChild(document.createTextNode(it + '\n'))}
function setColor(c)
  {document.querySelector('div').setAttribute('style','border-color:' + c)}
</script>
</head>
<body onload="window.getSelection().selectAllChildren(document.querySelector('p'))" ondragleave="leavePage(event)">
<p ondragstart="start(event)" ondragend="endDrag(event)">Drag me</p>
<p>Drag selected text out of frame and drop it somewhere on the page. Both circles should turn green once text is dropped.</p>
<div/>
<pre/>
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
        "byte_end": 167,
        "byte_start": 144,
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
        "byte_end": 314,
        "byte_start": 276,
        "col": 1,
        "line": 13
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
  "source_name": "html/editing/dnd/events/helper-drag-me-p-with-circle.xhtml"
}
```
