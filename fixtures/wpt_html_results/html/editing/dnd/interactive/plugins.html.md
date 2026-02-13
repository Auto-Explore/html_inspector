# html/editing/dnd/interactive/plugins.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/interactive/plugins.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
  <head>
    <title>drag and drop should not remove styling of plugin object elements</title>
    <style type="text/css">
div {
  border: 10px solid orange;
  background: yellow;
  padding: 10px;
  height: 140px;
  width: 140px;
}
object {
  border: 10px solid gray;
  background: fuchsia;
  padding: 10px;
  height: 100px;
  width: 100px;
}
    </style>
  </head>
  <body>

    <p>Drag the following block by the orange border. The drag placeholder should contain all inner borders, but may optionally show white or pink instead of the navy square.</p>
    <div draggable="true" ondragstart="event.dataTransfer.setData('Text','dummy text');"><object data="../resources/boxnavy.swf"></object></div>

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
  "source_name": "html/editing/dnd/interactive/plugins.html"
}
```
