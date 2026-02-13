# html/canvas/offscreen/manual/layers/unclosed-layers.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/offscreen/manual/layers/unclosed-layers.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html class="reftest-wait">
<link rel="match" href="unclosed-layers-expected.html">
<title>Canvas test: unclosed-layers</title>
<h1>unclosed-layers</h1>
<p class="desc">Check that unclosed layers aren't rendered.</p>
<canvas id="canvas" width="1" height="1">
  <p class="fallback">FAIL (fallback content)</p>
</canvas>
<script>
  var placeholder = document.getElementById('canvas');
  var offscreen = placeholder.transferControlToOffscreen();
  const ctx = offscreen.getContext('2d');
  offscreen.width = offscreen.height = 200;

  ctx.fillStyle = 'purple';
  ctx.fillRect(60, 60, 75, 50);

  ctx.beginLayer();
  ctx.fillRect(40, 40, 75, 50);
  ctx.fillStyle = 'grey';
  ctx.fillRect(50, 50, 75, 50);

  function draw () {
    // Wait until frame propagates.
    if(placeholder.width != 200) {
      requestAnimationFrame(draw);
    } else {
      document.documentElement.classList.remove("reftest-wait");
    }
  }
  requestAnimationFrame(draw);
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
  "source_name": "html/canvas/offscreen/manual/layers/unclosed-layers.html"
}
```
