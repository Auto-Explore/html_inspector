# html/semantics/embedded-content/the-img-element/image-loading-lazy-srcset.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/image-loading-lazy-srcset.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<head>
<title>loading='lazy' image with srcset</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/images.html#update-the-image-data">
<link rel="help" href="https://html.spec.whatwg.org/multipage/images.html#will-lazy-load-image-steps">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
</head>
<div style="height:1000vh;"></div>
<img srcset="resources/image.png?loading-lazy-srcset" loading="lazy">
<img loading="lazy" srcset="resources/image.png?loading-lazy-srcset">
<script>
promise_test(async t => {
  let loaded_images = 0;
  const imgs = document.querySelectorAll("img");
  imgs.forEach(img => {
    img.addEventListener("load", () => { loaded_images++; }, { once: true });
  });

  await new Promise(resolve => window.addEventListener("load", resolve));

  assert_equals(loaded_images, 0,
                "lazy-load images with srcset shouldn't be loaded yet");

  const promises = [
    new Promise(resolve => imgs[0].addEventListener("load", resolve)),
    new Promise(resolve => imgs[1].addEventListener("load", resolve)),
  ];

  imgs[1].scrollIntoView();
  await Promise.all(promises);

  imgs.forEach(img => {
    assert_true(img.complete,
                "Now the lazy-load image with srcset should be loaded");
  });
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
        "byte_end": 490,
        "byte_start": 421,
        "col": 1,
        "line": 10
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 560,
        "byte_start": 491,
        "col": 1,
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
  "source_name": "html/semantics/embedded-content/the-img-element/image-loading-lazy-srcset.html"
}
```
