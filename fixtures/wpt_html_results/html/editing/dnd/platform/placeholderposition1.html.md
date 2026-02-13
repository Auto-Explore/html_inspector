# html/editing/dnd/platform/placeholderposition1.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/platform/placeholderposition1.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
  <head>
    <title>drag and drop - position of the placeholder for a dragged element</title>
    <style type="text/css">
div { background: orange; color: black; width: 200px; padding: 10px; border: 10px solid orange; margin: 10px; }
    </style>
  </head>
  <body>

    <div draggable="true">Drag the orange block around the page (and only over the page), using the pixel in its top-left corner. When dragging, the top-left corner of the dragged placeholder should exactly match the position of the mouse cursor.</div>

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
        "byte_end": 144,
        "byte_start": 121,
        "col": 5,
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
  "source_name": "html/editing/dnd/platform/placeholderposition1.html"
}
```
