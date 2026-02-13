# html/canvas/element/manual/shadows/canvas_shadows_system_colors.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/manual/shadows/canvas_shadows_system_colors.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
  <meta charset="utf-8">
  <title>System Colors work for Canvas Drop-Shadow Filters</title>
  <link rel="match" href="canvas_shadows_system_colors-expected.html">
</head>
<body>
<canvas id='c' width="100" height="200">
<script>
// See crbug.com/1226282 and crbug.com/1081945
// A reference test is necessary because system colors do not have defined
// numeric values. Here we're comparing 'GrayText' to css 'GrayText'.
var ctx = document.getElementById('c').getContext('2d');
ctx.filter = 'drop-shadow(0px 100px 0 GrayText)';
ctx.fillRect(0,0,100,100);
</script>
</body>
</html>
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
  "source_name": "html/canvas/element/manual/shadows/canvas_shadows_system_colors.html"
}
```
