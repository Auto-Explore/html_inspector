# html/canvas/element/manual/filters/tentative/canvas-filter-object-blur-expected.html

Counts:
- errors: 4
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/manual/filters/tentative/canvas-filter-object-blur-expected.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<body>
  <canvas id="canvas" width="300" height="300"></canvas>
</body>
<script>
  var canvas = document.getElementById('canvas');
  var ctx = canvas.getContext('2d');
  ctx.filter = 'blur(2px)';
  ctx.fillStyle = 'yellow';
  ctx.fillRect(10,10,100,100);
  ctx.filter = 'blur(5px)';
  ctx.fillStyle = 'magenta';
  ctx.fillRect(120, 10, 100, 100);
  ctx.filter = 'blur(5px) blur(10px)';
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
        "byte_end": 80,
        "byte_start": 72,
        "col": 1,
        "line": 4
      }
    },
    {
      "category": "Html",
      "code": "html.parser.nonspace_after_body",
      "message": "Non-space character after body.",
      "severity": "Error",
      "span": {
        "byte_end": 533,
        "byte_start": 80,
        "col": 9,
        "line": 4
      }
    },
    {
      "category": "Html",
      "code": "html.parser.end_tag_after_body_closed",
      "message": "Saw an end tag after “body” had been closed.",
      "severity": "Error",
      "span": {
        "byte_end": 542,
        "byte_start": 533,
        "col": 1,
        "line": 19
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
  "source_name": "html/canvas/element/manual/filters/tentative/canvas-filter-object-blur-expected.html"
}
```
