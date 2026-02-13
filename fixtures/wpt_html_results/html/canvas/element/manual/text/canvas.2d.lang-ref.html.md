# html/canvas/element/manual/text/canvas.2d.lang-ref.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/manual/text/canvas.2d.lang-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype HTML>
<html class="reftest-wait">
  <meta charset="utf-8">
  <title>HTML5 Canvas Test Reference: The lang attribute.</title>
  <link rel="author" href="mailto:schenney@chromium.org"/>
  <script src="/common/reftest-wait.js"></script>
  <style>
    #canvas-en {
      position: absolute;
      top: 10px;
      left: 10px;
    }
    #canvas-tr {
      position: absolute;
      top: 120px;
      left: 10px;
    }
  </style>
  <script>
    function drawText(language) {
      var canvas = document.getElementById('canvas-' + language);
      var ctx = canvas.getContext('2d');

      ctx.font = '25px Lato-Medium';
      ctx.fillText('fi', 5, 50);
    }
    function generateReference() {
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
    }
  </script>
  <body onload="generateReference()">
    <canvas lang="en" id="canvas-tr" width="300" height="100">
      Browser does not support HTML5 Canvas.
    </canvas>
    <canvas lang="tr" id="canvas-en" width="300" height="100">
      Browser does not support HTML5 Canvas.
    </canvas>
  </body>
</html>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/canvas/element/manual/text/canvas.2d.lang-ref.html"
}
```
