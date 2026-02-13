# html/canvas/element/manual/filters/tentative/canvas-filter-object-component-transfer.html

Counts:
- errors: 4
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/manual/filters/tentative/canvas-filter-object-component-transfer.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<head>
    <link rel="match" href="canvas-filter-object-component-transfer-expected.html">
</head>
<body>
  <canvas id="canvas" width="500" height="100"></canvas>
</body>
<script>
  const ctx = document.getElementById("canvas").getContext("2d");

  const grad = ctx.createLinearGradient(10, 0, 490, 0);
  grad.addColorStop(0, "#f00");
  grad.addColorStop(0.33, "#0f0");
  grad.addColorStop(0.67, "#00f");
  grad.addColorStop(1, "#000");
  ctx.fillStyle = grad;

  const identityFilter = new CanvasFilter({
    name: "componentTransfer",
    funcR: {type: "identity"},
    funcG: {type: "identity"},
    funcB: {type: "identity"},
    funcA: {type: "identity"},
  });
  ctx.filter = identityFilter;
  ctx.fillRect(10, 10, 480, 10);

  const tableFilter = new CanvasFilter({
    name: "componentTransfer",
    funcR: {type: "table", tableValues: [0, 2, 0.5, 1]},
    funcG: {type: "table", tableValues: [1, -1, 5, 0]},
    funcB: {type: "table", tableValues: [0, 1, 1, 0]},
  });
  ctx.filter = tableFilter;
  ctx.fillRect(10, 30, 480, 10);

  const discreteFilter = new CanvasFilter({
    name: "componentTransfer",
    funcR: {type: "discrete", tableValues: [0, 2, 0.5, 1]},
    funcG: {type: "discrete", tableValues: [1, -1, 5, 0]},
    funcB: {type: "discrete", tableValues: [0, 1, 1, 0]},
  });
  ctx.filter = discreteFilter;
  ctx.fillRect(10, 50, 480, 10);

  const linearFilter = new CanvasFilter({
    name: "componentTransfer",
    funcR: {type: "linear", slope: 0.5, intercept: 0.25},
    funcG: {type: "linear", slope: 1.5, intercept: 0},
    funcB: {type: "linear", slope: -0.5, intercept: 0.5},
  });
  ctx.filter = linearFilter;
  ctx.fillRect(10, 70, 480, 10);

  const gammaFilter = new CanvasFilter({
    name: "componentTransfer",
    funcR: {type: "gamma", amplitude: 2, exponent: 5, offset: -0.5},
    funcG: {type: "gamma", amplitude: 0.9, exponent: 3, offset: 0.3},
    funcB: {type: "gamma", amplitude: 1.1, exponent: 1, offset: 0.1},
  });
  ctx.filter = gammaFilter;
  ctx.fillRect(10, 90, 480, 10);
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
        "byte_end": 179,
        "byte_start": 171,
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
        "byte_end": 2024,
        "byte_start": 179,
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
        "byte_end": 2033,
        "byte_start": 2024,
        "col": 1,
        "line": 62
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
  "source_name": "html/canvas/element/manual/filters/tentative/canvas-filter-object-component-transfer.html"
}
```
