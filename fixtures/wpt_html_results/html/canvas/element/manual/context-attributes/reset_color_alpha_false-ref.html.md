# html/canvas/element/manual/context-attributes/reset_color_alpha_false-ref.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/manual/context-attributes/reset_color_alpha_false-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>CanvasRenderingContext 2D with alpha=flase context creation parameter is re-initialized with solid black.</title>
<p>Test passes if a 100x100 black square is displayed below.</p>
<canvas id="c" width=100 height=100></canvas>
<script>
const ctx = document.getElementById("c").getContext("2d");
ctx.fillStyle = 'black';
ctx.fillRect(-1, -1, 102, 102);
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
  "source_name": "html/canvas/element/manual/context-attributes/reset_color_alpha_false-ref.html"
}
```
