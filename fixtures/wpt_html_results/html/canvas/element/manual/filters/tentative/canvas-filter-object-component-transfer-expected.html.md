# html/canvas/element/manual/filters/tentative/canvas-filter-object-component-transfer-expected.html

Counts:
- errors: 4
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/manual/filters/tentative/canvas-filter-object-component-transfer-expected.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<body>
  <canvas id="canvas" width="500" height="100"></canvas>
  <svg width="0", height="0">
    <defs>
      <filter color-interpolation-filters='sRGB' id="Identity" filterUnits="objectBoundingBox"
              x="0%" y="0%" width="100%" height="100%">
        <feComponentTransfer>
          <feFuncR type="identity"/>
          <feFuncG type="identity"/>
          <feFuncB type="identity"/>
          <feFuncA type="identity"/>
        </feComponentTransfer>
      </filter>
      <filter color-interpolation-filters='sRGB' id="Table">
        <feComponentTransfer>
          <feFuncR type="table" tableValues="0 2 0.5 1"/>
          <feFuncG type="table" tableValues="1 -1 5 0"/>
          <feFuncB type="table" tableValues="0 1 1 0"/>
        </feComponentTransfer>
      </filter>
      <filter color-interpolation-filters='sRGB' id="Discrete">
        <feComponentTransfer>
          <feFuncR type="discrete" tableValues="0 2 0.5 1"/>
          <feFuncG type="discrete" tableValues="1 -1 5 0"/>
          <feFuncB type="discrete" tableValues="0 1 1 0"/>
        </feComponentTransfer>
      </filter>
      <filter color-interpolation-filters='sRGB' id="Linear">
        <feComponentTransfer>
          <feFuncR type="linear" slope=".5" intercept=".25"/>
          <feFuncG type="linear" slope="1.5" intercept="0"/>
          <feFuncB type="linear" slope="-0.5" intercept=".5"/>
        </feComponentTransfer>
      </filter>
      <filter color-interpolation-filters='sRGB' id="Gamma">
        <feComponentTransfer>
          <feFuncR type="gamma" amplitude="2" exponent="5" offset="-0.5"/>
          <feFuncG type="gamma" amplitude="0.9" exponent="3" offset="0.3"/>
          <feFuncB type="gamma" amplitude="1.1" exponent="1" offset="0.1"/>
        </feComponentTransfer>
      </filter>
    </defs>
  </svg>
</body>
<script type="text/javascript">
  const ctx = document.getElementById("canvas").getContext("2d");

  const grad = ctx.createLinearGradient(10, 0, 490, 0);
  grad.addColorStop(0, "#f00");
  grad.addColorStop(0.33, "#0f0");
  grad.addColorStop(0.67, "#00f");
  grad.addColorStop(1, "#000");
  ctx.fillStyle = grad;

  ctx.filter = "url('#Identity')";
  ctx.fillRect(10, 10, 480, 10);
  ctx.filter = "url('#Table')";
  ctx.fillRect(10, 30, 480, 10);
  ctx.filter = "url('#Discrete')";
  ctx.fillRect(10, 50, 480, 10);
  ctx.filter = "url('#Linear')";
  ctx.fillRect(10, 70, 480, 10);
  ctx.filter = "url('#Gamma')";
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
        "byte_end": 1861,
        "byte_start": 1830,
        "col": 1,
        "line": 45
      }
    },
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 1861,
        "byte_start": 1830,
        "col": 1,
        "line": 45
      }
    },
    {
      "category": "Html",
      "code": "html.parser.nonspace_after_body",
      "message": "Non-space character after body.",
      "severity": "Error",
      "span": {
        "byte_end": 2476,
        "byte_start": 1861,
        "col": 32,
        "line": 45
      }
    },
    {
      "category": "Html",
      "code": "html.parser.end_tag_after_body_closed",
      "message": "Saw an end tag after “body” had been closed.",
      "severity": "Error",
      "span": {
        "byte_end": 2485,
        "byte_start": 2476,
        "col": 1,
        "line": 65
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
  "source_name": "html/canvas/element/manual/filters/tentative/canvas-filter-object-component-transfer-expected.html"
}
```
