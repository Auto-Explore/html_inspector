# html/editing/dnd/images/helper-drop-horizontal-scrollbar.xhtml

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/images/helper-drop-horizontal-scrollbar.xhtml",
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
  <script>
    function dropImage(event) {
      document.querySelector('div').firstChild.nodeValue = 'PASS';
    }
  </script>
</head>
<body>
  <div ondragenter="event.preventDefault()" ondragover="return false" ondrop="dropImage(event)">↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓
  </div>
</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/editing/dnd/images/helper-drop-horizontal-scrollbar.xhtml"
}
```
