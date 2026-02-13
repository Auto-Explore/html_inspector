# html/canvas/element/manual/filters/canvas-filter-shadow-and-properties-expected.html

Counts:
- errors: 3
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/manual/filters/canvas-filter-shadow-and-properties-expected.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<body>
  <canvas id="canvas" width="300" height="300"></canvas>
</body>
<script>
  /*
  The expected behavior when both shadow properties and filters are used at the
  same time is that the filter is applied to the elements drawn and the shadow
  properties create another shadow that includes even shadows of the
   filter-generated shadows.
  */
  var canvas = document.getElementById('canvas');
  var ctx = canvas.getContext('2d');
  ctx.fillStyle = 'cyan';
  ctx.fillRect(40, 40, 50, 50);
  ctx.fillRect(30, 30, 50, 50);
  ctx.fillStyle = 'red';
  ctx.fillRect(20, 20, 50, 50);
  ctx.fillStyle = 'black';
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
        "byte_end": 96,
        "byte_start": 88,
        "col": 1,
        "line": 5
      }
    },
    {
      "category": "Html",
      "code": "html.parser.nonspace_after_body",
      "message": "Non-space character after body.",
      "severity": "Error",
      "span": {
        "byte_end": 657,
        "byte_start": 96,
        "col": 9,
        "line": 5
      }
    },
    {
      "category": "Html",
      "code": "html.parser.end_tag_after_body_closed",
      "message": "Saw an end tag after “body” had been closed.",
      "severity": "Error",
      "span": {
        "byte_end": 666,
        "byte_start": 657,
        "col": 1,
        "line": 21
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
  "source_name": "html/canvas/element/manual/filters/canvas-filter-shadow-and-properties-expected.html"
}
```
