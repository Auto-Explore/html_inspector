# html/canvas/element/manual/filters/canvas-filter-shadow-and-properties-blur-expected.html

Counts:
- errors: 3
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/manual/filters/canvas-filter-shadow-and-properties-blur-expected.html",
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
  The shadow and shadow blur effects should be the same regardless if they were
  defined with filters or properties. The blur parameter is set as double when
  using the shadowBlur property since its uses havlf of the value set as the
  standard deviation for the gaussian blur (https://html.spec.whatwg.org/multipage/canvas.html#dom-context-2d-shadowblur-dev)
  while the filter parameter is used directly as the standard deviation
  (https://drafts.fxtf.org/filter-effects/#funcdef-filter-drop-shadow). The
  fuzziness is defined with a maxDifference of 13 as to be the 5% of 256, since
  the CSS spec defines the expected behavior in relation to an ideal Gaussian blur
   with a tolerance of 5%. See: https://drafts.csswg.org/css-backgrounds-3/#shadow-blur.
  */
  var canvas = document.getElementById('canvas');
  var ctx = canvas.getContext('2d');
  ctx.shadowOffsetX = 10;
  ctx.shadowOffsetY = 10;
  ctx.shadowBlur = 4;
  ctx.shadowColor = 'red';
  ctx.fillRect(20, 20, 50, 50);
  ctx.shadowBlur = 8;
  ctx.shadowColor = 'blue';
  ctx.fillRect(100, 20, 50, 50);
  ctx.shadowBlur = 20;
  ctx.shadowColor = 'yellow';
  ctx.fillRect(20, 100, 50, 50);
  ctx.shadowBlur = 30;
  ctx.shadowColor = 'cyan';
  ctx.fillRect(100, 100, 50, 50);
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
        "byte_end": 1343,
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
        "byte_end": 1352,
        "byte_start": 1343,
        "col": 1,
        "line": 33
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
  "source_name": "html/canvas/element/manual/filters/canvas-filter-shadow-and-properties-blur-expected.html"
}
```
