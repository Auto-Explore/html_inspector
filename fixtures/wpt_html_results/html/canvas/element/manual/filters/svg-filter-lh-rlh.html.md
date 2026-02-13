# html/canvas/element/manual/filters/svg-filter-lh-rlh.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/manual/filters/svg-filter-lh-rlh.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html class="test-wait">
<title>HTML Canvas test: SVG filter using CSS font-relative line-height units</title>
<link rel="match" href="svg-filter-lh-rlh-expected.html"/>
<link rel="stylesheet" type="text/css" href="/fonts/ahem.css" />
<style>
:root {
  font: 20px Ahem;
}
</style>
<svg width="0" height="0">
  <!-- https://html.spec.whatwg.org/multipage/canvas.html#working-with-externally-defined-svg-filters -->
  <use href="./svg-filter.svg#filter-lh" />
  <use href="./svg-filter.svg#filter-rlh" />
</svg
><canvas id="canvas-lh" width="100" height="100"></canvas><br
><canvas id="canvas-rlh" width="100" height="100"></canvas>
<script>
function use_filter(canvas, filter_id) {
  const ctx = canvas.getContext("2d");
  ctx.font = "40px Ahem";
  ctx.filter = `url(./svg-filter.svg#${filter_id})`;
  ctx.fillStyle = "red";
  ctx.fillRect(0, 0, 100, 100);
}
window.addEventListener("load", async () => {
  use_filter(document.getElementById("canvas-lh"), "filter-lh");
  use_filter(document.getElementById("canvas-rlh"), "filter-rlh");
  document.documentElement.classList.remove('test-wait');
});
</script>
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
  "source_name": "html/canvas/element/manual/filters/svg-filter-lh-rlh.html"
}
```
