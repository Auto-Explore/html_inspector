# html/editing/dnd/navigation/helper-drop-link-now.xhtml

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/navigation/helper-drop-link-now.xhtml",
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
html, body
  {height:100%;}
</style>
<script type="application/ecmascript">
function checkLink(event)
  {document.querySelector('p').firstChild.nodeValue = (event.dataTransfer.getData('text/uri-list').replace(/\r\n$/,'') == 'data:text/plain,1')?'PASS':'FAIL'}
</script>
</head>
<body ondragenter="event.preventDefault()" ondrop="checkLink(event)" ondragover="return false">
<p>Drop link now, you should see word PASS once you drop it.</p>
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
        "byte_end": 242,
        "byte_start": 204,
        "col": 1,
        "line": 9
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
  "source_name": "html/editing/dnd/navigation/helper-drop-link-now.xhtml"
}
```
