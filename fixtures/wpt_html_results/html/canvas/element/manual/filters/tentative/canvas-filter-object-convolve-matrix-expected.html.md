# html/canvas/element/manual/filters/tentative/canvas-filter-object-convolve-matrix-expected.html

Counts:
- errors: 3
- warnings: 10
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/manual/filters/tentative/canvas-filter-object-convolve-matrix-expected.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<style type="text/css">
  canvas {
    margin: 5px;
  }
</style>
<body>
  <svg width="0" height="0">
    <filter color-interpolation-filters='sRGB' id="justKernel">
      <feConvolveMatrix
          kernelMatrix="3 0 0 0 0 0 0 0 -3"/>
    </filter>
    <filter color-interpolation-filters='sRGB' id="preserveAlpha">
      <feConvolveMatrix
          kernelMatrix="3 0 0 0 0 0 0 0 -3"
          preserveAlpha="true"/>
    </filter>
    <filter color-interpolation-filters='sRGB' id="target">
      <feConvolveMatrix
          kernelMatrix="3 0 0 0 0 0 0 0 -3"
          targetX="2" targetY="2"/>
    </filter>
    <filter color-interpolation-filters='sRGB' id="divisor">
      <feConvolveMatrix
          kernelMatrix="3 0 0 0 0 0 0 0 -3"
          divisor="3"/>
    </filter>
    <filter color-interpolation-filters='sRGB' id="bias">
      <feConvolveMatrix
          kernelMatrix="3 0 0 0 0 0 0 0 -3"
          bias="0.5"/>
    </filter>
    <filter color-interpolation-filters='sRGB' id="edgeMode">
      <feConvolveMatrix
          kernelMatrix="3 0 0 0 0 0 0 0 -3"
          edgeMode="wrap"/>
    </filter>
  </svg>
</body>
<script type="text/javascript">

const filters = [
  "url('#justKernel')",
  "url('#preserveAlpha')",
  "url('#target')",
  "url('#divisor')",
  "url('#bias')",
  "url('#edgeMode')",
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

for (f of filters) {
  const canvas = document.createElement("canvas");
  document.body.prepend(canvas);
  const ctx = canvas.getContext("2d");
  ctx.filter = "blur(0px)";
  ctx.fillStyle = "rgba(0,255,0,0.5)";
  draw(ctx);
  ctx.fillStyle = "rgba(255,0,255,0.5)";
  ctx.filter = f;
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
        "byte_end": 39,
        "byte_start": 16,
        "col": 1,
        "line": 2
      }
    },
    {
      "category": "Html",
      "code": "html.svg.element.feconvolvematrix.missing_order",
      "message": "Element “feConvolveMatrix” is missing required attribute “order”.",
      "severity": "Warning",
      "span": {
        "byte_end": 250,
        "byte_start": 187,
        "col": 7,
        "line": 10
      }
    },
    {
      "category": "Html",
      "code": "html.svg.element.feconvolvematrix.missing_order",
      "message": "Element “feConvolveMatrix” is missing required attribute “order”.",
      "severity": "Warning",
      "span": {
        "byte_end": 432,
        "byte_start": 338,
        "col": 7,
        "line": 14
      }
    },
    {
      "category": "Html",
      "code": "html.svg.element.feconvolvematrix.missing_order",
      "message": "Element “feConvolveMatrix” is missing required attribute “order”.",
      "severity": "Warning",
      "span": {
        "byte_end": 610,
        "byte_start": 513,
        "col": 7,
        "line": 19
      }
    },
    {
      "category": "Html",
      "code": "html.svg.element.feconvolvematrix.missing_order",
      "message": "Element “feConvolveMatrix” is missing required attribute “order”.",
      "severity": "Warning",
      "span": {
        "byte_end": 777,
        "byte_start": 692,
        "col": 7,
        "line": 24
      }
    },
    {
      "category": "Html",
      "code": "html.svg.element.feconvolvematrix.missing_order",
      "message": "Element “feConvolveMatrix” is missing required attribute “order”.",
      "severity": "Warning",
      "span": {
        "byte_end": 940,
        "byte_start": 856,
        "col": 7,
        "line": 29
      }
    },
    {
      "category": "Html",
      "code": "html.svg.element.feconvolvematrix.missing_order",
      "message": "Element “feConvolveMatrix” is missing required attribute “order”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1112,
        "byte_start": 1023,
        "col": 7,
        "line": 34
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_start_tag",
      "message": "Stray start tag “script”.",
      "severity": "Error",
      "span": {
        "byte_end": 1175,
        "byte_start": 1144,
        "col": 1,
        "line": 40
      }
    },
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 1175,
        "byte_start": 1144,
        "col": 1,
        "line": 40
      }
    },
    {
      "category": "Html",
      "code": "html.parser.nonspace_after_body",
      "message": "Non-space character after body.",
      "severity": "Error",
      "span": {
        "byte_end": 1886,
        "byte_start": 1175,
        "col": 32,
        "line": 40
      }
    },
    {
      "category": "Html",
      "code": "html.parser.end_tag_after_body_closed",
      "message": "Saw an end tag after “body” had been closed.",
      "severity": "Error",
      "span": {
        "byte_end": 1895,
        "byte_start": 1886,
        "col": 1,
        "line": 77
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
  "source_name": "html/canvas/element/manual/filters/tentative/canvas-filter-object-convolve-matrix-expected.html"
}
```
