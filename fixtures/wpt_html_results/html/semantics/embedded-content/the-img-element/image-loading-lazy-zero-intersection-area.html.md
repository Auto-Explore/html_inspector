# html/semantics/embedded-content/the-img-element/image-loading-lazy-zero-intersection-area.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/image-loading-lazy-zero-intersection-area.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Image with zero intersection area is lazy-loaded</title>
<link rel="author" title="Emilio Cobos Álvarez" href="mailto:emilio@crisal.io">
<link rel="author" title="Mozilla" href="https://mozilla.org">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#the-img-element">
<link rel="help" href="https://html.spec.whatwg.org/#lazy-load-intersection-observer">
<link rel="help" href="https://bugzilla.mozilla.org/show_bug.cgi?id=1785186">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div style="height: 0; overflow: hidden;">
  <img style="display: block" id=target loading="lazy" width="100" height="100">
</div>
<script>
  async_test(function(t) {
    target.addEventListener("load", t.step_func_done(function() {}));
    target.src = "resources/image.png?zero-intersection-area";
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
        "byte_end": 722,
        "byte_start": 644,
        "col": 3,
        "line": 12
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 722,
        "byte_start": 644,
        "col": 3,
        "line": 12
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
  "source_name": "html/semantics/embedded-content/the-img-element/image-loading-lazy-zero-intersection-area.html"
}
```
