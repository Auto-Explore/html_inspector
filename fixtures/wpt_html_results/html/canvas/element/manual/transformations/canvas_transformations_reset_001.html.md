# html/canvas/element/manual/transformations/canvas_transformations_reset_001.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/manual/transformations/canvas_transformations_reset_001.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<link rel="match" href="canvas_transformations_reset_001-ref.html">
<style>
  html, body {
    margin: 0;
    padding: 0;
  }
</style>
<canvas id="c" width="150" height="150"></canvas>
<script>
var c = document.getElementById("c");
var ctx = c.getContext("2d");

ctx.translate(75, 75);
ctx.fillStyle = 'blue';
ctx.fillRect(0, 0, 75, 75);

ctx.resetTransform();
ctx.fillStyle = 'red';
ctx.fillRect(0, 0, 75, 75);
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
  "source_name": "html/canvas/element/manual/transformations/canvas_transformations_reset_001.html"
}
```
