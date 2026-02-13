# html/canvas/element/manual/filters/tentative/canvas-filter-object-turbulence.html

Counts:
- errors: 4
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/manual/filters/tentative/canvas-filter-object-turbulence.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<head>
    <link rel="match" href="canvas-filter-object-turbulence-expected.html">
</head>
<body>
</body>
<script>
  const testCases = [
    {baseFrequency: 0.03125},
    {baseFrequency: [0.03125, 0.125]},
    {baseFrequency: 0.0625},
    {baseFrequency: 0.03125, seed: 100},
    {baseFrequency: 0.03125, numOctaves: 2},
    {},
    {baseFrequency: 0.03125, type: "fractalNoise"},
    {baseFrequency: 0.03125, stitchTiles: "stitch"},
  ]

  for (tc of testCases) {
    const canvas = document.createElement("canvas");
    document.body.appendChild(canvas);
    const ctx = canvas.getContext("2d");
    const filterOptions = {...{name: "turbulence"}, ...tc};
    ctx.filter = new CanvasFilter(filterOptions);
    ctx.fillRect(0, 0, 1, 1);
  }
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
        "byte_end": 114,
        "byte_start": 106,
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
        "byte_end": 742,
        "byte_start": 114,
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
        "byte_end": 751,
        "byte_start": 742,
        "col": 1,
        "line": 26
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
  "source_name": "html/canvas/element/manual/filters/tentative/canvas-filter-object-turbulence.html"
}
```
