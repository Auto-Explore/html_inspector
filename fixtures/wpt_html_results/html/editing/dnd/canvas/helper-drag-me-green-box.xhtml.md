# html/editing/dnd/canvas/helper-drag-me-green-box.xhtml

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/canvas/helper-drag-me-green-box.xhtml",
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
<style type="text/css">
div
  {width:20px;
  height:20px;
  background-color:green;}
body
  /* Center the div in this iframe since we know the iframe size (it's fixed
  * in the parent page to 'width:300px; height:200px;') */
  {margin-top:90px;
  margin-left:140px;}
</style>
<script type="application/ecmascript">
function start(event)
  {event.dataTransfer.effectAllowed = 'copy';
  event.dataTransfer.setData('text/plain', 'green');}
</script>
</head>
<body>
<div draggable="true" ondragstart="start(event)"/></body>
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
        "byte_end": 162,
        "byte_start": 139,
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
        "byte_end": 454,
        "byte_start": 416,
        "col": 1,
        "line": 16
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
  "source_name": "html/editing/dnd/canvas/helper-drag-me-green-box.xhtml"
}
```
