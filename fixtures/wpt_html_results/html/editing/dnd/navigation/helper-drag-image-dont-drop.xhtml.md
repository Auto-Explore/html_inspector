# html/editing/dnd/navigation/helper-drag-image-dont-drop.xhtml

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/navigation/helper-drag-image-dont-drop.xhtml",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<?xml version="1.0" encoding="utf-8"?>
<html xmlns="http://www.w3.org/1999/xhtml">
<head>
<title>Cross page drag and drop: helper file</title>
<style type="text/css">
div[ondragenter]
  {margin:200px 0 0 200px;
  width:200px;
  height:100px;
  color:white;
  background-color:navy;}
div[ondragenter]:before
  {display:block;
  content:"";
  border-style:solid;
  position:relative;
  top:-50px;
  left:-200px;
  border-width:100px;
  border-color:transparent navy transparent transparent;}
</style>
</head>
<body>
<p>Drag image to the blue arrow but don't drop it yet. You should be returned back to start page.</p>
<div ondragenter="event.preventDefault()" ondragover="history.go(-1)"/>
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
        "byte_end": 166,
        "byte_start": 143,
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
  "source_name": "html/editing/dnd/navigation/helper-drag-image-dont-drop.xhtml"
}
```
