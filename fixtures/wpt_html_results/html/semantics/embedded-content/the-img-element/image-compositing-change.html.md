# html/semantics/embedded-content/the-img-element/image-compositing-change.html

Counts:
- errors: 2
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/image-compositing-change.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html class="reftest-wait">
<title>Composited images correctly re-raster when the image and bounds change</title>
<meta charset="utf-8">
<meta name=fuzzy content="maxDifference=0-150;totalPixels=0-296">
<link rel="match" href="image-compositing-change-ref.html"/>
<link rel="help" href="https://html.spec.whatwg.org/multipage/#the-img-element">
<style>
#change {
  will-change:transform;
  height:426px; width:426px;
}
</style>
<img id="change" src="image.png"></img>
<img id="original" src="../../../../images/green-16x16.png"></img>
<script>
window.onload = () => {
  requestAnimationFrame(() => {
    requestAnimationFrame(() => {
      let image = document.querySelector('#change');
      image.style.width = image.style.height = "75px";
      image.src = original.src;

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
        "byte_end": 477,
        "byte_start": 444,
        "col": 1,
        "line": 14
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “img”.",
      "severity": "Error",
      "span": {
        "byte_end": 483,
        "byte_start": 477,
        "col": 34,
        "line": 14
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 544,
        "byte_start": 484,
        "col": 1,
        "line": 15
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “img”.",
      "severity": "Error",
      "span": {
        "byte_end": 550,
        "byte_start": 544,
        "col": 61,
        "line": 15
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
  "source_name": "html/semantics/embedded-content/the-img-element/image-compositing-change.html"
}
```
