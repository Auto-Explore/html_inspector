# html/canvas/offscreen/manual/the-offscreen-canvas/offscreencanvas-worker-font-load-crash.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/offscreen/manual/the-offscreen-canvas/offscreencanvas-worker-font-load-crash.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html class="test-wait">
<script>
  let url = URL.createObjectURL(new Blob([`
    let font = new FontFace('Ahem', 'url(/fonts/Ahem.ttf)');
    self.fonts.add(font);
    let canvas = new OffscreenCanvas(100, 100);
    let ctx = canvas.getContext('2d');
    ctx.font = "10px Ahem";
    ctx.fillText('Hello', 0, 10);
    postMessage('done');
  `], { type: "application/javascript" }));
  var worker = new Worker(url);
  worker.onmessage = function() {
    worker.terminate();
    URL.revokeObjectURL(url);
    document.documentElement.className = "";
  };
</script>
```

```json
{
  "messages": [
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
  "source_name": "html/canvas/offscreen/manual/the-offscreen-canvas/offscreencanvas-worker-font-load-crash.html"
}
```
