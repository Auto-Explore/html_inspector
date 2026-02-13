# html/canvas/element/manual/text/canvas.2d.lang.inherit.disconnected.canvas.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/manual/text/canvas.2d.lang.inherit.disconnected.canvas.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype HTML>
<html lang="en-US" class="reftest-wait">
  <meta charset="utf-8">
  <title>HTML5 Canvas Test: The lang attribute inherits from a disconnected canvas element</title>
  <link rel="match" href="canvas.2d.lang-ref.html"/>
  <link rel="author" href="mailto:schenney@chromium.org"/>
  <link rel="help"
    href="https://html.spec.whatwg.org/multipage/canvas.html#text-styles"/>
  <meta name="assert" content="Verify that a disconnected canvas uses the canvas lang."/>
  <script src="/common/reftest-wait.js"></script>
  <style>
    #canvas-tr {
      position: absolute;
      top: 10px;
      left: 10px;
    }
    #canvas-en {
      position: absolute;
      top: 120px;
      left: 10px;
    }
  </style>
  <body>
    <script type="text/javascript">
      function drawText(language) {
        var canvas = document.createElement('canvas');
        canvas.setAttribute('width', '300');
        canvas.setAttribute('height', '100');
        canvas.setAttribute('id', 'canvas-' + language);
        canvas.setAttribute('lang', language);
        var ctx = canvas.getContext('2d');

        // The default for lang is inherit
        ctx.font = '25px Lato-Medium';
        ctx.fillText('fi', 5, 50);

        document.body.appendChild(canvas);
      }

      let test_font = new FontFace(
        // Lato-Medium is a font with language specific ligatures.
        "Lato-Medium",
        "url(/fonts/Lato-Medium.ttf)"
      );

      test_font.load().then((font) => {
        document.fonts.add(font);
        drawText('tr');
        drawText('en');
        takeScreenshot();
      });
    </script>
  </body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 762,
        "byte_start": 731,
        "col": 5,
        "line": 24
      }
    }
  ],
  "source_name": "html/canvas/element/manual/text/canvas.2d.lang.inherit.disconnected.canvas.html"
}
```
