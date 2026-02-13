# html/semantics/embedded-content/the-img-element/image-loading-lazy-clip-path.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/image-loading-lazy-clip-path.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html class="reftest-wait">
<link rel="author" title="Xianzhu Wang" href="mailto:wangxianzhu@chromium.org">
<link rel="help" href="https://html.spec.whatwg.org/multipage/urls-and-fetching.html#lazy-loading-attributes">
<link rel="help" href="https://crbug.com/1308299">
<link rel="match" href="image-loading-lazy-clip-path-ref.html">
<script src="/common/reftest-wait.js"></script>
<img id=target loading="lazy"
      src="resources/image.png"
      style="vertical-align: middle; clip-path: polygon(0 0, 110% 0, 110% 110%, 0 110%, 0 0)">
<script>
  target.onload = takeScreenshot;
</script>
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
        "byte_end": 554,
        "byte_start": 398,
        "col": 1,
        "line": 8
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
  "source_name": "html/semantics/embedded-content/the-img-element/image-loading-lazy-clip-path.html"
}
```
