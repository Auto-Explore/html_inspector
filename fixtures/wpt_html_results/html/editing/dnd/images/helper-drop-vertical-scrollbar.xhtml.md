# html/editing/dnd/images/helper-drop-vertical-scrollbar.xhtml

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/images/helper-drop-vertical-scrollbar.xhtml",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<?xml version="1.0" encoding="utf-8"?>
<html xmlns="http://www.w3.org/1999/xhtml">
<head>
  <title>Image drag and drop: helper file</title>
  <style type="text/css">
    div {
      width: 1ex;
    }
  </style>
  <script>
    function dropImage(event) {
      document.querySelector('div').firstChild.nodeValue = 'PASS';
    }
  </script>
</head>
<body>
  <div ondragenter="event.preventDefault()" ondragover="return false" ondrop="dropImage(event)">→ → → → → → →</div>
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
        "byte_end": 165,
        "byte_start": 142,
        "col": 3,
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
  "source_name": "html/editing/dnd/images/helper-drop-vertical-scrollbar.xhtml"
}
```
