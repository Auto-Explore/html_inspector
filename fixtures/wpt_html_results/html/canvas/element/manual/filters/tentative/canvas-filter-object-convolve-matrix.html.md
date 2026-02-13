# html/canvas/element/manual/filters/tentative/canvas-filter-object-convolve-matrix.html

Counts:
- errors: 3
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/manual/filters/tentative/canvas-filter-object-convolve-matrix.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<head>
    <link rel="match" href="canvas-filter-object-convolve-matrix-expected.html">
    <style type="text/css">
      canvas {
        margin: 5px;
      }
    </style>
</head>
<body>
</body>
<script>
function makeConvolveFilter(options) {
  const KERNEL_MATRIX = [
    [3, 0, 0],
    [0, 0, 0],
    [0, 0, -3],
  ];

  options = Object.assign(options, {
    kernelMatrix: KERNEL_MATRIX, name: "convolveMatrix"});
  return new CanvasFilter(options);
}

const test_cases = [
  {},
  {preserveAlpha: true},
  {targetX: 2, targetY: 2},
  {divisor: 3},
  {bias: 0.5},
  {edgeMode: "wrap"}
];

function draw(ctx) {
  ctx.fillRect(0, 20, 120, 100);

  ctx.beginPath();
  ctx.arc(150, 70, 50, 0, 2*Math.PI);
  ctx.fill();

  ctx.beginPath();
  ctx.moveTo(220, 20);
  ctx.lineTo(170, 120);
  ctx.lineTo(270, 120);
  ctx.lineTo(220, 20);
  ctx.fill();
}

for (tc of test_cases) {
  const canvas = document.createElement("canvas");
  document.body.prepend(canvas);
  const ctx = canvas.getContext("2d");
  ctx.filter = "blur(0px)";
  ctx.fillStyle = "rgba(0,255,0,0.5)";
  draw(ctx);
  ctx.fillStyle = "rgba(255,0,255,0.5)";
  ctx.filter = makeConvolveFilter(tc);
  draw(ctx);
}
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.style.type.unnecessary",
      "message": "The “type” attribute for the “style” element is not needed and should be omitted.",
      "severity": "Warning",
      "span": {
        "byte_end": 131,
        "byte_start": 108,
        "col": 5,
        "line": 4
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_start_tag",
      "message": "Stray start tag “script”.",
      "severity": "Error",
      "span": {
        "byte_end": 220,
        "byte_start": 212,
        "col": 1,
        "line": 12
      }
    },
    {
      "category": "Html",
      "code": "html.parser.nonspace_after_body",
      "message": "Non-space character after body.",
      "severity": "Error",
      "span": {
        "byte_end": 1189,
        "byte_start": 220,
        "col": 9,
        "line": 12
      }
    },
    {
      "category": "Html",
      "code": "html.parser.end_tag_after_body_closed",
      "message": "Saw an end tag after “body” had been closed.",
      "severity": "Error",
      "span": {
        "byte_end": 1198,
        "byte_start": 1189,
        "col": 1,
        "line": 60
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
  "source_name": "html/canvas/element/manual/filters/tentative/canvas-filter-object-convolve-matrix.html"
}
```
