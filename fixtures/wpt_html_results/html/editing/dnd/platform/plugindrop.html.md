# html/editing/dnd/platform/plugindrop.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/platform/plugindrop.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
  <head>
    <title>drag and drop sequence should end when dropping over a plugin</title>
    <style type="text/css">
div {
  background: orange;
  height: 100px;
  width: 100px;
}
object {
  height: 100px;
  width: 100px;
}
    </style>
  </head>
  <body>

    <p>Use your pointing device to drag the orange square onto the blue square, and release it. The drag placeholder should disappear after releasing (or as the pointer moves over the blue square). Try dragging the orange square again. Fail if it does not respond when trying to drag it.</p>
    <div draggable="true" ondragstart="event.dataTransfer.setData('Text','dummy text');"></div>
    <p><object data="../resources/boxnavy.swf"></object></p>

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
        "byte_end": 140,
        "byte_start": 117,
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
  "source_name": "html/editing/dnd/platform/plugindrop.html"
}
```
