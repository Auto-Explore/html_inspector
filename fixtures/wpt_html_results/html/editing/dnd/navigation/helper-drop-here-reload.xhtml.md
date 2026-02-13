# html/editing/dnd/navigation/helper-drop-here-reload.xhtml

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/navigation/helper-drop-here-reload.xhtml",
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
p
  {border:solid medium navy;
  height:200px;
  padding:1em;
  margin:0;}
div
  {margin:100px;
  padding:50px;}
img
  {display:block;
  margin:1em;}
</style>
<script type="application/ecmascript">
function addImage(event)
  {var c = document.createElement('img');
  c.setAttribute('src',event.dataTransfer.getData('text/uri-list').replace(/\r\n$/,''));
  document.querySelector('p').appendChild(c);}
</script>
</head>
<body>
<div ondragenter="window.location.reload()">
<p ondragenter="event.stopPropagation()" dropzone="copy string:text/uri-list" ondrop="addImage(event)">Drop image here, it should be copied to this page once you drop it here.</p>
</div>
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
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 364,
        "byte_start": 326,
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
  "source_name": "html/editing/dnd/navigation/helper-drop-here-reload.xhtml"
}
```
