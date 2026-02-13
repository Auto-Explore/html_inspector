# html/canvas/offscreen/manual/layers/unclosed-nested-layers.w.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/offscreen/manual/layers/unclosed-nested-layers.w.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html class="reftest-wait">
<link rel="match" href="unclosed-nested-layers-expected.html">
<title>Canvas test: unclosed-nested-layers</title>
<h1>unclosed-nested-layers</h1>
<p class="desc">Check that unclosed nested layers aren't rendered.</p>
<canvas id="canvas" width="1" height="1">
  <p class="fallback">FAIL (fallback content)</p>
</canvas>
<script id='myWorker' type='text/worker'>
  self.onmessage = msg => {
    const offscreen = msg.data.canvas;
    const ctx = offscreen.getContext('2d');
    offscreen.width = offscreen.height = 200;

    ctx.fillStyle = 'rgba(0, 0, 255, 1)';
    ctx.fillRect(60, 60, 75, 50);

    ctx.beginLayer();
    ctx.fillStyle = 'rgba(225, 0, 0, 1)';
    ctx.fillRect(50, 50, 75, 50);

    ctx.beginLayer();
    ctx.fillStyle = 'rgba(0, 255, 0, 1)';
    ctx.fillRect(70, 70, 75, 50);

    ctx.endLayer();
    // Missing ctx.endLayer() here.

    self.postMessage('setup ready');
  }
</script>
<script>
  const blob = new Blob([document.getElementById('myWorker').textContent]);
  const worker = new Worker(URL.createObjectURL(blob));
  var placeholder = document.getElementById('canvas');
  var offscreen = placeholder.transferControlToOffscreen();
  worker.addEventListener('message', msg => {
    if(msg.data == 'setup ready') {
      function draw () {
          // Wait until frame propagates.
          if(placeholder.width != 1) {
            document.documentElement.classList.remove("reftest-wait");
          } else {
            requestAnimationFrame(draw);
          }
      }
      requestAnimationFrame(draw);
    }
  });
  worker.postMessage({
      canvas: offscreen
  }, [offscreen]);
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
  "source_name": "html/canvas/offscreen/manual/layers/unclosed-nested-layers.w.html"
}
```
