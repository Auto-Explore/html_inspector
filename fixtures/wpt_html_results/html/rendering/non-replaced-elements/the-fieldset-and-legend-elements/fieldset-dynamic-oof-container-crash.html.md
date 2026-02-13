# html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-dynamic-oof-container-crash.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-dynamic-oof-container-crash.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html class="test-wait">
<title>No crash after stop being an asbolute container.</title>
<link rel="help" href="https://crbug.com/1395688">
<style>
.c2 {
  transform: rotate3d(0, 1, 0, 45deg);
  column-width: 100px;
}
.c19 {
  overflow: auto;
  padding-left: 65536px;
  column-count: 3;
}

q {
  position: absolute;
  column-width: 1px;
}

body {
  column-count: 3;
}
</style>
<script>
function animationFrame() {
  return new Promise(resolve => requestAnimationFrame(resolve));
}

async function doTest() {
  document.documentElement.appendChild(document.createElement('body'));
  await animationFrame();
  document.body.innerHTML = '<fieldset class=c2><q>q</q></fieldset>';
  window.scrollBy(28, 71);
  await animationFrame();
  document.querySelector('fieldset').setAttribute('class', 'c19');
  await animationFrame();
  document.body.remove();
  await animationFrame();
  await animationFrame();
  document.documentElement.classList.remove('test-wait');
}

doTest();
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
  "source_name": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-dynamic-oof-container-crash.html"
}
```
