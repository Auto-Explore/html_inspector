# html/canvas/element/manual/drawing-text-to-the-canvas/canvas.2d.disconnected-font-size-math.html

Counts:
- errors: 3
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/manual/drawing-text-to-the-canvas/canvas.2d.disconnected-font-size-math.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>font-size: math treated as medium in disconnected canvas</title>
<link rel="match" href="canvas.2d.disconnected-font-size-math-ref.html">
<body>
</body>
<script>
var d = new Document();
var c = d.createElementNS("http://www.w3.org/1999/xhtml", "canvas");
var ctx = c.getContext("2d");
ctx.font = `math serif`;
ctx.fillText("Hello World!", 5, c.height / 2);
c.toBlob((blob) => {
  var img = document.createElement("img");
  const url = URL.createObjectURL(blob);
  img.src = url;
  img.style.border = "3px solid cyan";
  document.body.appendChild(img);
});
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.stray_start_tag",
      "message": "Stray start tag “script”.",
      "severity": "Error",
      "span": {
        "byte_end": 207,
        "byte_start": 199,
        "col": 1,
        "line": 7
      }
    },
    {
      "category": "Html",
      "code": "html.parser.nonspace_after_body",
      "message": "Non-space character after body.",
      "severity": "Error",
      "span": {
        "byte_end": 602,
        "byte_start": 207,
        "col": 9,
        "line": 7
      }
    },
    {
      "category": "Html",
      "code": "html.parser.end_tag_after_body_closed",
      "message": "Saw an end tag after “body” had been closed.",
      "severity": "Error",
      "span": {
        "byte_end": 611,
        "byte_start": 602,
        "col": 1,
        "line": 20
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
  "source_name": "html/canvas/element/manual/drawing-text-to-the-canvas/canvas.2d.disconnected-font-size-math.html"
}
```
