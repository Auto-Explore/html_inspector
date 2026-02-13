# html/canvas/element/manual/filters/tentative/canvas-filter-object-blur.html

Counts:
- errors: 4
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/manual/filters/tentative/canvas-filter-object-blur.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<head>
    <link rel="match" href="canvas-filter-object-blur-expected.html">
</head>
<body>
  <canvas id="canvas" width="300" height="300"></canvas>
</body>
<script>
  var canvas = document.getElementById('canvas');
  var ctx = canvas.getContext('2d');
  ctx.filter = new CanvasFilter({name: "gaussianBlur", stdDeviation: 2});
  ctx.fillStyle = 'yellow';
  ctx.fillRect(10,10,100,100);
  ctx.filter = new CanvasFilter({name: "gaussianBlur", stdDeviation: 5});
  ctx.fillStyle = 'magenta';
  ctx.fillRect(120, 10, 100, 100);
  ctx.filter = new CanvasFilter([
    {name: "gaussianBlur", stdDeviation: 5},
    {name: "gaussianBlur", stdDeviation: 10}]);
  ctx.fillStyle = 'cyan';
  ctx.fillRect(10, 120, 100, 100);
  ctx.filter = 'none';
  ctx.fillStyle = 'black';
  ctx.fillRect(120, 120, 100, 100);
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.start_tag_without_doctype",
      "message": "Start tag seen without seeing a doctype first. Expected “<!DOCTYPE html>”.",
      "severity": "Error",
      "span": {
        "byte_end": 6,
        "byte_start": 0,
        "col": 1,
        "line": 1
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_start_tag",
      "message": "Stray start tag “script”.",
      "severity": "Error",
      "span": {
        "byte_end": 165,
        "byte_start": 157,
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
        "byte_end": 798,
        "byte_start": 165,
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
        "byte_end": 807,
        "byte_start": 798,
        "col": 1,
        "line": 24
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
  "source_name": "html/canvas/element/manual/filters/tentative/canvas-filter-object-blur.html"
}
```
