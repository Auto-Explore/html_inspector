# html/canvas/element/manual/filters/tentative/idl-conversions/canvas-filter-long-conversion.html

Counts:
- errors: 4
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/manual/filters/tentative/idl-conversions/canvas-filter-long-conversion.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<head>
    <link rel="match" href="canvas-filter-long-conversion-expected.html">
</head>
<body>
  <canvas id="canvas" width="300" height="300"></canvas>
</body>
<script>
  // Test the built-in ECMAScript types Undefined, Null, Boolean, String, Number, and Object
  // as input to the CanvasFilter resolver when a long is the intended result.
  var ctx = document.getElementById('canvas').getContext('2d');

  // Null, False and [] evaluate to zero
  ctx.filter  = new CanvasFilter({filter: "gaussianBlur", stdDeviation: null});
  ctx.fillRect(10, 10, 30, 30);
  ctx.filter  = new CanvasFilter({filter: "gaussianBlur", stdDeviation: false});
  ctx.fillRect(50, 10, 30, 30);
  ctx.filter  = new CanvasFilter({filter: "gaussianBlur", stdDeviation: []});
  ctx.fillRect(90, 10, 30, 30);
  // True evaluates to one
  ctx.filter  = new CanvasFilter({filter: "gaussianBlur", stdDeviation: true});
  ctx.fillRect(130, 10, 30, 30);
  // String, Number and Object should all work
  ctx.filter  = new CanvasFilter({filter: "gaussianBlur", stdDeviation: "5"});
  ctx.fillRect(10, 50, 30, 30);
  ctx.filter  = new CanvasFilter({filter: "gaussianBlur", stdDeviation: 5});
  ctx.fillRect(50, 50, 30, 30);
  ctx.filter  = new CanvasFilter({filter: "gaussianBlur", stdDeviation: { valueOf() { return 5; }}});
  ctx.fillRect(90, 50, 30, 30);
  // Valid sequence
  ctx.filter  = new CanvasFilter({filter: "gaussianBlur", stdDeviation: [5]});
  ctx.fillRect(130, 50, 30, 30);

  // Undefined and other inputs that throw exceptions are tested in:
  // html/canvas/element/filters/2d.filter.canvasFilterObject.blur.exceptions.html
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
        "byte_end": 169,
        "byte_start": 161,
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
        "byte_end": 1609,
        "byte_start": 169,
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
        "byte_end": 1618,
        "byte_start": 1609,
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
  "source_name": "html/canvas/element/manual/filters/tentative/idl-conversions/canvas-filter-long-conversion.html"
}
```
