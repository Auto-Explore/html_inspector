# html/semantics/embedded-content/the-img-element/image-loading-lazy-data-url-to-https.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/image-loading-lazy-data-url-to-https.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html class="reftest-wait">
<head>
  <title>Lazy loaded Images with data url placeholders can be overwritten by a src change</title>
  <link rel="help" href="https://html.spec.whatwg.org/multipage/images.html#update-the-image-data">
  <link rel="match" href="image-loading-lazy-data-url-to-https-ref.html">
  <script src="/common/reftest-wait.js"></script>
</head>

<body>
  <img id="image" loading="lazy" src="data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 872 490' width='872' height='490' style='background: green' %3E%3C/svg%3E">

<script>
  const image = document.querySelector('#image');

  window.onload = function() {
    // trigger intersection observer through forced layout.
    image.offsetWidth;
    image.setAttribute("src", 'resources/image.png');
    setTimeout(() => { takeScreenshot(); }, 100);
  };
</script>
</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.img.src.invalid",
      "message": "Bad value “data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 872 490' width='872' height='490' style='background: green' %3E%3C/svg%3E” for attribute “src” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 576,
        "byte_start": 391,
        "col": 3,
        "line": 11
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 576,
        "byte_start": 391,
        "col": 3,
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
  "source_name": "html/semantics/embedded-content/the-img-element/image-loading-lazy-data-url-to-https.html"
}
```
