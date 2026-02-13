# html/editing/dnd/selection/helper-drag-me-input-to-other-textarea.xhtml

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/selection/helper-drag-me-input-to-other-textarea.xhtml",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<?xml version="1.0" encoding="utf-8"?>
<html xmlns="http://www.w3.org/1999/xhtml">
<head>
<title>Selection drag and drop: helper file</title>
</head>
<body onload="document.querySelector('input').select()">
<p>Drag selected text to the textarea. Copy of selection should end up in the textarea once you drop it there.</p>
<p><input value="Drag me"/></p>
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
  "source_name": "html/editing/dnd/selection/helper-drag-me-input-to-other-textarea.xhtml"
}
```
