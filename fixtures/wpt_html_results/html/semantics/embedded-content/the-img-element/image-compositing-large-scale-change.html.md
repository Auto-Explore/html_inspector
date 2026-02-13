# html/semantics/embedded-content/the-img-element/image-compositing-large-scale-change.html

Counts:
- errors: 1
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/image-compositing-large-scale-change.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html class="reftest-wait">
<title>Composited images correctly display under large scale transform changes</title>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width,initial-scale=1,minimum-scale=1">
<link rel="match" href="image-compositing-large-scale-change-ref.html"/>
<link rel="help" href="https://html.spec.whatwg.org/multipage/#the-img-element">
<style>
html { overflow: hidden; }
#change {
  will-change:transform;
  width:1000px;
  height:1000px;
  position:absolute;
  top: calc(50% - 5px);
  left: calc(50% - 5px);
}
</style>
<img id="change" src="image.png"></img>
<div id="placeholder" style="position:relative"></div>
<script>
window.onload = () => {
  requestAnimationFrame(() => {
    requestAnimationFrame(() => {
      let image = document.querySelector('#change');
      image.style.transform = 'scale(20)';
      placeholder.innerText = "div";

      requestAnimationFrame(() => {
        document.documentElement.classList.remove("reftest-wait");
      });
    });
  });
}
</script>

```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 609,
        "byte_start": 576,
        "col": 1,
        "line": 19
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “img”.",
      "severity": "Error",
      "span": {
        "byte_end": 615,
        "byte_start": 609,
        "col": 34,
        "line": 19
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
  "source_name": "html/semantics/embedded-content/the-img-element/image-compositing-large-scale-change.html"
}
```
