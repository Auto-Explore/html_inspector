# html/canvas/offscreen/manual/layers/unclosed-nested-layers-expected.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/offscreen/manual/layers/unclosed-nested-layers-expected.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Canvas test: unclosed-nested-layers</title>
<h1>unclosed-nested-layers</h1>
<p class="desc">Check that unclosed nested layers aren't rendered.</p>
<canvas id="canvas" width="200" height="200">
  <p class="fallback">FAIL (fallback content)</p>
</canvas>
<script>
  const canvas = document.getElementById("canvas");
  const ctx = canvas.getContext('2d');

  ctx.fillStyle = 'rgba(0,0,255,1)';
  ctx.fillRect(60,60,75,50);
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
  "source_name": "html/canvas/offscreen/manual/layers/unclosed-nested-layers-expected.html"
}
```
