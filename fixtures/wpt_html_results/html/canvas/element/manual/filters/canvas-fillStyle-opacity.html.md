# html/canvas/element/manual/filters/canvas-fillStyle-opacity.html

Counts:
- errors: 3
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/manual/filters/canvas-fillStyle-opacity.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel="match" href="canvas-opacity-expected.html"/>
<meta name="fuzzy" content="maxDifference=0-2; totalPixels=0-14000">
<body>
  <canvas id="canvas" width="300" height="300"></canvas>
</body>
<script>
  var canvas = document.getElementById('canvas');
  var ctx = canvas.getContext('2d');
  ctx.fillStyle = 'rgba(0, 128, 128, 0.75)';
  ctx.fillRect(10, 10, 50, 50);
  ctx.fillStyle = 'rgba(255, 0, 255, 0.5)';
  ctx.fillRect(70, 10, 50, 50);
  ctx.fillStyle = 'rgba(255, 165, 0, 0.25)';
  ctx.fillRect(10, 70, 50, 50);
  ctx.fillStyle = 'rgba(210, 105, 30, 0)';
  ctx.fillRect(70, 70, 50, 50);

  ctx.fillStyle = 'rgba(0, 255, 255, 0.8)';
  ctx.fillRect(10, 150, 50, 50);
  ctx.fillStyle = 'rgba(255, 0, 0, 0.6)';
  ctx.fillRect(20, 150, 50, 50);
  ctx.fillStyle = 'rgba(255, 255, 0, 0.4)';
  ctx.fillRect(30, 150, 50, 50);
  ctx.fillStyle = 'rgba(0, 128, 0, 0.2)';
  ctx.fillRect(40, 150, 50, 50);
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
        "byte_end": 221,
        "byte_start": 213,
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
        "byte_end": 919,
        "byte_start": 221,
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
        "byte_end": 928,
        "byte_start": 919,
        "col": 1,
        "line": 27
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
  "source_name": "html/canvas/element/manual/filters/canvas-fillStyle-opacity.html"
}
```
