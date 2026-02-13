# html/semantics/embedded-content/the-img-element/resources/image-loading-lazy-in-viewport.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/resources/image-loading-lazy-in-viewport.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>

<!-- This frame is used by image-loading-lazy-in-cross-origin-iframe-002.sub.html -->

<img id="img" loading="lazy">

<script>
  const img = document.querySelector('#img');

  // Prevent the list of available images check from loading the image non-lazily.
  img.src = "image.png?image-loading-lazy-in-viewport-" + Math.random() + "-" + Date.now();

  img.addEventListener("load", () => {
    parent.postMessage("image_loaded", "*");
  });

  window.addEventListener("load", () => {
    parent.postMessage("window_loaded", "*");
  });
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 133,
        "byte_start": 104,
        "col": 1,
        "line": 5
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 133,
        "byte_start": 104,
        "col": 1,
        "line": 5
      }
    },
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
  "source_name": "html/semantics/embedded-content/the-img-element/resources/image-loading-lazy-in-viewport.html"
}
```
