# html/semantics/embedded-content/the-img-element/image-loading-lazy-available.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/image-loading-lazy-available.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>The list of available images gets checked before deciding to make a load lazy</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/images.html#update-the-image-data">
<link rel="help" href="https://html.spec.whatwg.org/multipage/images.html#will-lazy-load-image-steps">
<link rel="help" href="https://bugzilla.mozilla.org/show_bug.cgi?id=1709577">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<img src="/images/green-256x256.png">
<div style="height:1000vh;"></div>
<script>
promise_test(async t => {
  await new Promise(resolve => {
    window.addEventListener("load", resolve);
  });
  let nonLazy = document.querySelector("img");
  assert_equals(nonLazy.width, 256);
  assert_equals(nonLazy.height, 256);

  let lazy = document.createElement("img");
  lazy.loading = "lazy";
  lazy.src = nonLazy.src;
  document.body.appendChild(lazy);

  await new Promise(resolve => setTimeout(resolve));

  assert_equals(lazy.width, 256, "The list of available images should be checked before delaying the image load");
  assert_equals(lazy.height, 256, "The list of available images should be checked before delaying the image load");
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
        "byte_end": 532,
        "byte_start": 495,
        "col": 1,
        "line": 9
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
  "source_name": "html/semantics/embedded-content/the-img-element/image-loading-lazy-available.html"
}
```
