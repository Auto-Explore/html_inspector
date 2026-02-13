# html/canvas/element/manual/filters/tentative/idl-conversions/canvas-filter-boolean-conversion.html

Counts:
- errors: 4
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/manual/filters/tentative/idl-conversions/canvas-filter-boolean-conversion.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<head>
    <link rel="match" href="canvas-filter-boolean-conversion-expected.html">
</head>
<body>
  <canvas id="canvas" width="300" height="300"></canvas>
</body>
<script>
  // Test the built-in ECMAScript types Undefined, Null, Boolean, String, Number, and Object
  // as input to the CanvasFilter resolver when a bool is the intended result.
  var ctx = document.getElementById('canvas').getContext('2d');

  // preserveAlpha for convolveMatrix is the only boolean so far implemented
  function drawWithConvolveFilter(x, y, preserveAlphaValue) {
    ctx.filter = new CanvasFilter({
      name: "convolveMatrix",
      kernelMatrix: [[1, 0], [0, 1]],
      preserveAlpha: preserveAlphaValue,
    });
    ctx.fillRect(x, y, 30, 30);
  }

  const trueTestCases = [
    true,
    { valueOf() { return false; }},
    "foo",
    1,
    {},
    []
  ];

  const falseTestCases = [
    false,
    "",
    0,
    null,
    undefined,
  ];

  ctx.fillStyle = "rgba(255,0,255,0.5)";
  let x = 10;
  let y = 10;
  for (tc of trueTestCases) {
    drawWithConvolveFilter(x, y, tc);
    x += 40;
  }
  y = 50;
  x = 10;
  for (tc of falseTestCases) {
    drawWithConvolveFilter(x, y, tc);
    x += 40;
  }

  ctx.filter = new CanvasFilter({
    name: "componentTransfer",
    funcR: {type: "discrete", tableValues: 0.5},
  });
  ctx.fillRect(10, 10, 100, 100);
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
        "byte_end": 172,
        "byte_start": 164,
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
        "byte_end": 1349,
        "byte_start": 172,
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
        "byte_end": 1358,
        "byte_start": 1349,
        "col": 1,
        "line": 58
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
  "source_name": "html/canvas/element/manual/filters/tentative/idl-conversions/canvas-filter-boolean-conversion.html"
}
```
