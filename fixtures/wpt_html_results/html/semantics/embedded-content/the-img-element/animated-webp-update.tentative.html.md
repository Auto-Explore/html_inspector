# html/semantics/embedded-content/the-img-element/animated-webp-update.tentative.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/animated-webp-update.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html class="reftest-wait">
  <head>
    <title>Animated image active frame update when passing delay time</title>
    <link rel="match" href="animated-image-update-ref.tentative.html" />
    <link rel="author" title="Tin Tun Aung" href="mailto:rayguo17@gmail.com" />
    <script src="/common/reftest-wait.js"></script>
  </head>

  <body>
    <img id="image" src="resources/animated.webp" />
    <script>
      window.addEventListener("load", go);
      function go() {
        requestAnimationFrame(() => {
          setTimeout(() => {
            takeScreenshot();
          }, 150);
        });
      }
    </script>
  </body>
</html>
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
        "byte_end": 408,
        "byte_start": 360,
        "col": 5,
        "line": 11
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
  "source_name": "html/semantics/embedded-content/the-img-element/animated-webp-update.tentative.html"
}
```
