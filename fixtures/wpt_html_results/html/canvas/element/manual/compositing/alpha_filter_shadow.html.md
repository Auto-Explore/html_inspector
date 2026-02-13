# html/canvas/element/manual/compositing/alpha_filter_shadow.html

Counts:
- errors: 1
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/manual/compositing/alpha_filter_shadow.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<body>
<script>
var canvas, ctx;
canvas = document.createElement("canvas");
canvas.width = 100;
canvas.height = 100;
document.body.appendChild(canvas);

ctx = canvas.getContext("2d");
ctx.globalAlpha = 0.5;
ctx.shadowOffsetX = -canvas.width;
ctx.shadowColor = 'red';
ctx.filter = 'sepia(0)';
ctx.fillRect(canvas.width,0,canvas.width,canvas.height);
</script>
</body>
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
        "byte_end": 6,
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
  "source_name": "html/canvas/element/manual/compositing/alpha_filter_shadow.html"
}
```
