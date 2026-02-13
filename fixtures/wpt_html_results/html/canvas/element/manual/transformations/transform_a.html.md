# html/canvas/element/manual/transformations/transform_a.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/manual/transformations/transform_a.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<link rel=match href=transform_ref.html>
<style>
html, body {
    margin: 0;
}
</style>
<canvas id=c width=400 height=300></canvas>
<script>
var canvas = document.getElementById('c');
var ctx = canvas.getContext('2d');
ctx.scale(3, 3);
ctx.fillStyle = 'rgb(255, 0, 0)';
ctx.beginPath();
ctx.moveTo(10, 10);
ctx.bezierCurveTo(10, 10, 20, 10, 20, 10);
ctx.bezierCurveTo(20, 10, 20, 20, 20, 20);
ctx.bezierCurveTo(20, 20, 10, 20, 10, 20);
ctx.closePath();
ctx.fill();
</script>
```

```json
{
  "messages": [
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
  "source_name": "html/canvas/element/manual/transformations/transform_a.html"
}
```
