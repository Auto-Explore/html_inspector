# html/canvas/element/manual/filters/canvas-opacity-blend-modes-expected.html

Counts:
- errors: 3
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/manual/filters/canvas-opacity-blend-modes-expected.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<body>
</body>
<script>
  /*
  Compare how the opacity is handled in different blend modes when setting its
  value with filters or with properties.
  */

  function drawSquares(canvasId, x, y, compositeOperation) {
    var canvas = document.getElementById(canvasId);
    var ctx = canvas.getContext('2d');
    canvas.style.position = 'absolute';
    canvas.style.left = `${x}px`;
    canvas.style.top = `${y}px`;

    ctx.globalCompositeOperation = 'source-over';
    ctx.globalAlpha = 1.0;
    ctx.fillStyle = 'green';
    ctx.fillRect(0, 0, 200, 60);
    ctx.fillStyle = 'blue';
    ctx.fillRect(0, 0, 50, 50);
    ctx.globalAlpha = 0.7;
    ctx.fillStyle = 'red';
    ctx.fillRect(50, 0, 50, 50);
    ctx.globalCompositeOperation = compositeOperation;
    ctx.globalAlpha = 0.5;
    ctx.fillStyle = 'yellow';
    ctx.fillRect(25, 25, 50, 50);
  }

  // Fomatted in the same matrix as the drawn elements.
  var compositeOperations =
    ['source-over', 'source-in', 'source-out', 'source-atop','destination-over',
    'destination-in', 'destination-out', 'destination-atop', 'lighter', 'copy',
    'xor', 'multiply', 'screen', 'overlay', 'darken',
    'lighten', 'color-dodge', 'color-burn', 'hard-light', 'soft-light',
    'difference', 'exclusion', 'hue', 'saturation', 'color',
    'luminosity'];

  for (var i = 0; i < compositeOperations.length; i++){
    var canvas = document.createElement('canvas');
    canvas.id = `canvas-${i}`;
    document.body.appendChild(canvas);
    drawSquares(canvas.id, (i%5)*300, Math.floor(i/5)*300,
      compositeOperations[i]);
  }
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
        "byte_end": 39,
        "byte_start": 31,
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
        "byte_end": 1591,
        "byte_start": 39,
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
        "byte_end": 1600,
        "byte_start": 1591,
        "col": 1,
        "line": 48
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
  "source_name": "html/canvas/element/manual/filters/canvas-opacity-blend-modes-expected.html"
}
```
