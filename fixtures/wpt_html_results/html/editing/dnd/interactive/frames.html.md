# html/editing/dnd/interactive/frames.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/interactive/frames.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
  <head>
    <title>drag and drop should allow dragging of iframes and object iframes</title>
    <style type="text/css">
iframe, object {
  border: 10px solid orange;
  background: blue;
  padding: 10px;
  height: 100px;
  width: 100px;
}
    </style>
  </head>
  <body>

    <p>It should be possible to drag the following two blocks by both their orange and blue borders.</p>
    <p><iframe draggable="true" src="frames-1.html"></iframe></p>
    <p><object draggable="true" data="frames-1.html"></object></p>

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
  "source_name": "html/editing/dnd/interactive/frames.html"
}
```
