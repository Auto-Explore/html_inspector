# html/canvas/element/manual/text/canvas.2d.fontStretch-ref.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/manual/text/canvas.2d.fontStretch-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Canvas test: 2d.text.fontStretch</title>
<canvas id="c" class="output"><p class="fallback">FAIL (fallback content)</p></canvas>
<script>

var canvas = document.getElementById("c");
var ctx = canvas.getContext('2d');

function draw() {
    ctx.font = '25px test';
    ctx.fillText("P", 10, 40);
}

var f = new FontFace('test', 'url(/fonts/pass.woff)');
document.fonts.add(f);

f.load().then(draw);

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
  "source_name": "html/canvas/element/manual/text/canvas.2d.fontStretch-ref.html"
}
```
