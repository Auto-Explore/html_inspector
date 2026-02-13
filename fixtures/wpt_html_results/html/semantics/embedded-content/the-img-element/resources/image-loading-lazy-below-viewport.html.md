# html/semantics/embedded-content/the-img-element/resources/image-loading-lazy-below-viewport.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/resources/image-loading-lazy-below-viewport.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<div style="height:1000vh;"></div>

<img id="img" loading="lazy" src="image.png?lazy-below-viewport">

<script>
  const img = document.querySelector('#img');

  img.addEventListener("load", () => {
    parent.postMessage("image_loaded", "*");
  });

  window.addEventListener("load", () => {
    parent.postMessage("window_loaded", "*");
  });

  window.addEventListener("message", event => {
    if (event.data == "scroll") {
      img.scrollIntoView();
    }
  });
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
        "byte_end": 117,
        "byte_start": 52,
        "col": 1,
        "line": 4
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
  "source_name": "html/semantics/embedded-content/the-img-element/resources/image-loading-lazy-below-viewport.html"
}
```
