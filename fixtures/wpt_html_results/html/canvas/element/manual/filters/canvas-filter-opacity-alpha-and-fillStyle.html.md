# html/canvas/element/manual/filters/canvas-filter-opacity-alpha-and-fillStyle.html

Counts:
- errors: 3
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/manual/filters/canvas-filter-opacity-alpha-and-fillStyle.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel="match" href="canvas-filter-opacity-alpha-and-fillStyle-expected.html"/>
<meta name="fuzzy" content="maxDifference=0-2; totalPixels=0-10000">
<body>
  <canvas id="canvas" width="300" height="300"></canvas>
</body>
<script>
  /*
  The expected behavior when setting the opacity through different methods
  is that the opacity of the resulting drawn element is the product of the opacity
  value set by each of the methods.
  */
  var canvas = document.getElementById('canvas');
  var ctx = canvas.getContext('2d');

  ctx.globalAlpha = 0.5;
  ctx.fillStyle = 'rgba(0,0,0,0.5)';
  ctx.filter = 'opacity(50%)';
  ctx.fillRect(10, 10, 50, 50);

  ctx.globalAlpha = 0.5;
  ctx.fillStyle = 'rgba(0,0,0,0.25)';
  ctx.filter = 'opacity(25%)';
  ctx.fillRect(70, 10, 50, 50);

  ctx.globalAlpha = 0.75;
  ctx.fillStyle = 'rgba(0,0,0,0.5)';
  ctx.filter = 'opacity(50%)';
  ctx.fillRect(10, 70, 50, 50);

  ctx.globalAlpha = 0.8;
  ctx.fillStyle = 'rgba(0,0,0,0.2)';
  ctx.filter = 'opacity(10%)';
  ctx.fillRect(70, 70, 50, 50);
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
        "byte_end": 248,
        "byte_start": 240,
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
        "byte_end": 1046,
        "byte_start": 248,
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
        "byte_end": 1055,
        "byte_start": 1046,
        "col": 1,
        "line": 35
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
  "source_name": "html/canvas/element/manual/filters/canvas-filter-opacity-alpha-and-fillStyle.html"
}
```
