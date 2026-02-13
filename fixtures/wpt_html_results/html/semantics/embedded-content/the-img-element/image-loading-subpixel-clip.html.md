# html/semantics/embedded-content/the-img-element/image-loading-subpixel-clip.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/image-loading-subpixel-clip.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<style>
* {
  margin: 0;
}
</style>
<html class="reftest-wait" style="overflow: hidden">
  <head>
    <title>Images with loading='lazy' load under subpixel-offset clips</title>
    <link rel="author" title="Chris Harrelson" href="mailto:chrishtr@chromium.org">
    <link rel="help" href="https://html.spec.whatwg.org/#lazy-loading-attributes">
    <link rel="match" href="image-loading-subpixel-clip-ref.html">
  </head>
  <div style="height: 44.5px"></div>
  <div style="overflow: hidden">
    <div style="position: relative; font-size: 0; background: lightblue">
      <img id=target loading="lazy" data-sizes="auto">
    </div>
  </div>
</html>
<script src="/common/reftest-wait.js"></script>
<script>
  target.onload = takeScreenshot;
  target.src = "resources/image.png";
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
        "byte_end": 635,
        "byte_start": 587,
        "col": 7,
        "line": 17
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 635,
        "byte_start": 587,
        "col": 7,
        "line": 17
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
  "source_name": "html/semantics/embedded-content/the-img-element/image-loading-subpixel-clip.html"
}
```
