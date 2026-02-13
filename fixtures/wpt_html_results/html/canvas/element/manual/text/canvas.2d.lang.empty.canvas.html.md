# html/canvas/element/manual/text/canvas.2d.lang.empty.canvas.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/manual/text/canvas.2d.lang.empty.canvas.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype HTML>
<html class="reftest-wait">
  <meta charset="utf-8">
  <title>HTML5 Canvas Test: The empty string lang attribute uses the unknown language</title>
  <link rel="match" href="canvas.2d.lang.empty-ref.html" />
  <link rel="author" href="mailto:schenney@chromium.org"/>
  <link rel="help" href="https://html.spec.whatwg.org/multipage/canvas.html#text-styles"/>
  <meta name="assert" content="When the lang attribute is the empty string, use the unknown language." />
  <script src="/common/reftest-wait.js"></script>
  <script type="text/javascript">
    function runTest() {
      let test_font = new FontFace(
        // Lato-Medium is a font with language specific ligatures.
        "Lato-Medium",
        "url(/fonts/Lato-Medium.ttf)"
      );

      test_font.load().then((font) => {
        document.fonts.add(font);
        var canvas = document.getElementById('canvas1');
        var ctx = canvas.getContext('2d');

        // An empty lang string should produce the same result as no language
        // attribute at all, on the canvas element or document element.
        ctx.font = '25px Lato-Medium';
        ctx.lang = '';
        ctx.fillText('fi', 5, 50);
        takeScreenshot();
      });
    }
  </script>
  <body onload="runTest()">
    <canvas id="canvas1" width="300" height="150">
      Browser does not support HTML5 Canvas.
    </canvas>
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
        "byte_end": 562,
        "byte_start": 531,
        "col": 3,
        "line": 10
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/canvas/element/manual/text/canvas.2d.lang.empty.canvas.html"
}
```
