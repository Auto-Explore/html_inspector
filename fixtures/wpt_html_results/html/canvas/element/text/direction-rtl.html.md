# html/canvas/element/text/direction-rtl.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/text/direction-rtl.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>HTML Canvas Test: the 'direction' property</title>
<link rel="author" title="Jonathan Kew" href="mailto:jkew@mozilla.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/canvas.html#text-styles">
<link rel="match" href="reference/direction-rtl-ref.html">
<meta name="assert" content="text rendering respects the direction property">
<style>
canvas {
  margin: 10px;
  border: 1px solid gray;
}
</style>

<canvas id="c" width="300" height="300"></canvas>

<script>
let ctx = c.getContext("2d");
ctx.direction = "rtl";
ctx.font = "16px sans-serif";
ctx.textAlign = "left";
ctx.fillText("Hello World!", 150, 50);
ctx.textAlign = "right";
ctx.fillText("Hello World!", 150, 100);
ctx.textAlign = "start";
ctx.fillText("Hello World!", 150, 150);
ctx.textAlign = "end";
ctx.fillText("Hello World!", 150, 200);
ctx.textAlign = "center";
ctx.fillText("Hello World!", 150, 250);
</script>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/canvas/element/text/direction-rtl.html"
}
```
