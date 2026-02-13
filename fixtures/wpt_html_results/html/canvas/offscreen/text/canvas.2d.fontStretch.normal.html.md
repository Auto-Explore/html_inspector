# html/canvas/offscreen/text/canvas.2d.fontStretch.normal.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/offscreen/text/canvas.2d.fontStretch.normal.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Canvas test: 2d.text.fontStretch</title>
<link rel="match" href="canvas.2d.fontStretch-ref.html">
<canvas id="c" class="output"><p class="fallback">FAIL (fallback content)</p></canvas>
<script>

var canvas = new OffscreenCanvas(300, 150);
var ctx = canvas.getContext('2d');

// P shows as Pass for fontStretch = normal and shows as fail for
// fontStretch = expanded or condensed.
function draw() {
    ctx.font = '25px test';
    ctx.fillText("P", 10, 40);
    document.getElementById('c')
            .getContext("bitmaprenderer")
            .transferFromImageBitmap(canvas.transferToImageBitmap());
}

var f = new FontFace('test', 'url(/fonts/fail.woff)');
f.stretch = "expanded";
document.fonts.add(f);

var f1 = new FontFace('test', 'url(/fonts/pass.woff)');
document.fonts.add(f1);

var f2 = new FontFace('test', 'url(/fonts/fail.woff)');
f2.stretch = "condensed";
document.fonts.add(f2);

Promise.all([f.load(), f1.load(), f2.load()]).then(draw);

</script>
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
  "source_name": "html/canvas/offscreen/text/canvas.2d.fontStretch.normal.html"
}
```
