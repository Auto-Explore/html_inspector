# html/canvas/element/manual/filters/canvas-filter-shadow.html

Counts:
- errors: 3
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/manual/filters/canvas-filter-shadow.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel="match" href="canvas-filter-shadow-expected.html"/>
<body>
  <canvas id="canvas" width="300" height="300"></canvas>
</body>
<script>
  /*
  The expected behavior of filter-generated shadows is tested against
  a drawing using only rectangles that draws the shadows manually.
  */
  var canvas = document.getElementById('canvas');
  var ctx = canvas.getContext('2d');
  ctx.filter = 'drop-shadow(10px 10px 0 red)';
  ctx.fillRect(10, 10, 50, 50);
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
        "byte_end": 158,
        "byte_start": 150,
        "col": 1,
        "line": 6
      }
    },
    {
      "category": "Html",
      "code": "html.parser.nonspace_after_body",
      "message": "Non-space character after body.",
      "severity": "Error",
      "span": {
        "byte_end": 472,
        "byte_start": 158,
        "col": 9,
        "line": 6
      }
    },
    {
      "category": "Html",
      "code": "html.parser.end_tag_after_body_closed",
      "message": "Saw an end tag after “body” had been closed.",
      "severity": "Error",
      "span": {
        "byte_end": 481,
        "byte_start": 472,
        "col": 1,
        "line": 15
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
  "source_name": "html/canvas/element/manual/filters/canvas-filter-shadow.html"
}
```
