# html/editing/dnd/platform/placeholderposition2.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/platform/placeholderposition2.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
  <head>
    <title>drag and drop - position of the placeholder for a dragged selection</title>
    <style type="text/css">
p { cursor: default; }
    </style>
  </head>
  <body>

    <p>Select the first word in this sentence. Drag the selection downwards, using the pixel in the top-left corner of the selection highlight. When dragging, the top-left corner of the dragged placeholder should exactly match the position of the mouse cursor.</p>

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
        "byte_end": 146,
        "byte_start": 123,
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
  "source_name": "html/editing/dnd/platform/placeholderposition2.html"
}
```
