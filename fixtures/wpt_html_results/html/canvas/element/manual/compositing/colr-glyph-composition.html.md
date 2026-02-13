# html/canvas/element/manual/compositing/colr-glyph-composition.html

Counts:
- errors: 1
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/manual/compositing/colr-glyph-composition.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<html>
<meta charset="utf-8">
<link rel="author" title="Jonathan Kew" href="mailto:jkew@mozilla.com">
<link rel="help" href="https://bugzilla.mozilla.org/show_bug.cgi?id=1902253">
<link rel="match" href="colr-glyph-composition-ref.html">
<style>
canvas {
  margin: 5px;
}
</style>
<script>
function test(op) {
  cv = document.createElement("canvas");
  cv.width = "100";
  cv.height = "100";
  document.body.appendChild(cv);
  cx = cv.getContext("2d");
  cx.fillStyle = "#888888";
  cx.fillRect(0,0,100,50);
  cx.globalCompositeOperation=op;
  cx.fillStyle = "black";
  cx.font = "64px Segoe UI Emoji, sans-serif";
  cx.fillText("\u{1F63A}",5,65);
}

function br() {
  document.body.appendChild(document.createElement("br"));
}

window.onload = ( ) => {
  test("source-over");
  test("source-in");
  test("source-out");
  test("source-atop");
  br();
  test("destination-over");
  test("destination-in");
  test("destination-out");
  test("destination-atop");
  br();
  test("copy");
  test("lighter");
  test("xor");
  test("multiply");
  br();
  test("screen");
  test("overlay");
  test("darken");
  test("lighten");
  br();
  test("color-dodge");
  test("color-burn");
  test("hard-light");
  test("soft-light");
  br();
  test("difference");
  test("exclusion");
  test("hue");
  test("saturation");
  br();
  test("color");
  test("luminosity");
};
</script>
<body>
</body>
</html>
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
  "source_name": "html/canvas/element/manual/compositing/colr-glyph-composition.html"
}
```
