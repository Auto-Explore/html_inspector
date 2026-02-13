# html/semantics/embedded-content/the-img-element/image-loading-lazy-slow-aspect-ratio.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/image-loading-lazy-slow-aspect-ratio.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html class="reftest-wait">
  <link rel="match" href="image-loading-lazy-slow-aspect-ratio-ref.html">
  <link rel="author" title="Chris Harrelson" href="mailto:chrishtr@chromium.org">
  <link rel="help" href="https://html.spec.whatwg.org/multipage/#the-img-element">
  <script src="/common/reftest-wait.js"></script>
  <img id=target loading="lazy"
      width="200" height="100" style="width: 100px; height: auto; border: 1px solid black">
<script>
  let loaded = false;
  target.onload = () => {
    if (loaded) return;
    loaded = true;
    target.src = "";
    requestAnimationFrame(() => requestAnimationFrame(() => {
      target.src = "resources/image.png?slow-aspect-ratio&pipe=trickle(d2)";
      takeScreenshot();
    }));
  };
  target.src = "resources/image.png?slow-aspect-ratio";
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
        "byte_end": 456,
        "byte_start": 335,
        "col": 3,
        "line": 7
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 456,
        "byte_start": 335,
        "col": 3,
        "line": 7
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
  "source_name": "html/semantics/embedded-content/the-img-element/image-loading-lazy-slow-aspect-ratio.html"
}
```
