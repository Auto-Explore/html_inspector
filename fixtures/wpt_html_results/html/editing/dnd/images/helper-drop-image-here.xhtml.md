# html/editing/dnd/images/helper-drop-image-here.xhtml

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/images/helper-drop-image-here.xhtml",
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
    div[ondragenter] {
      width: 105px;
      min-height: 105px;
      text-align: center;
      margin-top: 20px;
      padding: 10px;
      border: solid thin navy;
    }
  </style>
  <script>
    function addImage(event) {
      var c = document.createElement('img');
      c.setAttribute('src', event.dataTransfer.getData('text/uri-list').replace(/\r\n$/, ''));
      document.querySelector('div').appendChild(c);
    }
  </script>
</head>
<body>
  <div ondragenter="event.preventDefault()" ondragover="return false" ondrop="addImage(event)"></div>
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
  "source_name": "html/editing/dnd/images/helper-drop-image-here.xhtml"
}
```
