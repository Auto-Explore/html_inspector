# html/editing/dnd/overlay/036-manual.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/overlay/036-manual.html",
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
<title>Drag feedback when multiple elements are added to dragstore</title>
<style type="text/css">
div > div
  {height:100px;
  width:100px;
  float:left;
  background-color:navy;}
div + div
  {margin-left:-60px;
  background-color:maroon;}
div[draggable]
  {background-color:teal;
  margin-top:50px;}
</style>
<script type="application/ecmascript">
function start(event)
  {var div = document.querySelectorAll('div > div:nth-child(odd)');
  event.dataTransfer.effectAllowed = 'copy';
  for(var i = 0; i != div.length; i++)
    {event.dataTransfer.addElement(div[i]);}
  }
</script>
</head>
<body>
<p>Try to drag green box below. Feedback overlay should include all three boxes when you drag green one.</p>
<div ondragstart="start(event)">
  <div></div>
  <div draggable="true"></div>
  <div></div>
</div>
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
      "code": "html.style.type.unnecessary",
      "message": "The “type” attribute for the “style” element is not needed and should be omitted.",
      "severity": "Warning",
      "span": {
        "byte_end": 204,
        "byte_start": 181,
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
        "byte_end": 455,
        "byte_start": 417,
        "col": 1,
        "line": 19
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
  "source_name": "html/editing/dnd/overlay/036-manual.html"
}
```
