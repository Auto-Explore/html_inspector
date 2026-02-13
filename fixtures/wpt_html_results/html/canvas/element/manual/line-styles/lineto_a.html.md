# html/canvas/element/manual/line-styles/lineto_a.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/manual/line-styles/lineto_a.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<link rel=match href=lineto_ref.html>
<style>
  html, body {
    margin: 0;
    padding: 0;
  }
</style>
<canvas id="c" width="150" height="150" >
Your browser does not support the HTML5 canvas tag.</canvas>

<script>
var c = document.getElementById("c");
var ctx = c.getContext("2d");

ctx.beginPath();
ctx.moveTo(20, 20);
ctx.lineTo(20, 130);
ctx.lineTo(130, 130);
ctx.lineTo(130, 20);
ctx.closePath();

ctx.fillStyle = '#90EE90';
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
  "source_name": "html/canvas/element/manual/line-styles/lineto_a.html"
}
```
