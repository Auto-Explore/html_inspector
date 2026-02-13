# html/canvas/element/manual/fill-and-stroke-styles/conic-gradient-rotation.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/manual/fill-and-stroke-styles/conic-gradient-rotation.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
  <title>Conic gradient</title>
  <link rel="match" href="conic-gradient-rotation-expected.html"/>
</head>
<body>
  <canvas id="c"></canvas>
  <script type="text/javascript">
    const canvas = document.getElementById('c');
    const ctx = canvas.getContext('2d');

    const grad = ctx.createConicGradient(2.5*Math.PI, 100, 50);

    grad.addColorStop(0, "red");
    grad.addColorStop(0.2, "red");
    grad.addColorStop(0.2, "orange");
    grad.addColorStop(0.4, "orange");
    grad.addColorStop(0.4, "yellow");
    grad.addColorStop(0.6, "yellow");
    grad.addColorStop(0.6, "green");
    grad.addColorStop(0.8, "green");
    grad.addColorStop(0.8, "blue");

    ctx.fillStyle = grad;
    ctx.fillRect(0, 0, canvas.width, canvas.height);
  </script>

</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 204,
        "byte_start": 173,
        "col": 3,
        "line": 9
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
  "source_name": "html/canvas/element/manual/fill-and-stroke-styles/conic-gradient-rotation.html"
}
```
