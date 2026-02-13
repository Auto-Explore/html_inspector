# html/semantics/embedded-content/the-img-element/image-loading-lazy-slow.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/image-loading-lazy-slow.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html class="reftest-wait">
  <link rel="match" href="image-loading-lazy-slow-ref.html">
  <link rel="author" title="Chris Harrelson" href="mailto:chrishtr@chromium.org">
  <link rel="help" href="https://html.spec.whatwg.org/multipage/#the-img-element">

  <script src="/common/reftest-wait.js"></script>
  <img id=target loading="lazy"
       width="330" height="254" style="border: 1px solid black">
<script>
  let loaded = false;
  target.onload = () => {
    if (loaded) return;
    loaded = true;
    target.src = "";
    requestAnimationFrame(() => requestAnimationFrame(() => {
      target.src = "resources/image.png?loading-lazy-slow&pipe=trickle(d2)";
      takeScreenshot();
    }));
  };
  target.src = "resources/image.png?loading-lazy-slow";
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
        "byte_end": 417,
        "byte_start": 323,
        "col": 3,
        "line": 8
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 417,
        "byte_start": 323,
        "col": 3,
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
  "source_name": "html/semantics/embedded-content/the-img-element/image-loading-lazy-slow.html"
}
```
