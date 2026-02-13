# html/canvas/element/manual/drawing-images-to-the-canvas/drawimage_canvas_self_ref.html

Counts:
- errors: 1
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/manual/drawing-images-to-the-canvas/drawimage_canvas_self_ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<canvas id="dest" height="100" width="100"></canvas>
<script>
var canvasWidth = canvasHeight = 100;
var destCanvas = document.getElementById('dest');
var destCtx = destCanvas.getContext('2d');
destCtx.fillStyle = 'red';
destCtx.fillRect(0, 0,  canvasWidth, canvasHeight);
destCtx.fillStyle = 'green';
destCtx.fillRect(0, 0, canvasWidth / 2, canvasHeight / 2);
destCtx.fillRect(canvasWidth / 2, canvasHeight / 2, canvasWidth / 4, canvasHeight / 4);
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.start_tag_without_doctype",
      "message": "Start tag seen without seeing a doctype first. Expected “<!DOCTYPE html>”.",
      "severity": "Error",
      "span": {
        "byte_end": 43,
        "byte_start": 0,
        "col": 1,
        "line": 1
      }
    },
    {
      "category": "Html",
      "code": "html.head.title.missing",
      "message": "Element “head” is missing a required instance of child element “title”.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/canvas/element/manual/drawing-images-to-the-canvas/drawimage_canvas_self_ref.html"
}
```
