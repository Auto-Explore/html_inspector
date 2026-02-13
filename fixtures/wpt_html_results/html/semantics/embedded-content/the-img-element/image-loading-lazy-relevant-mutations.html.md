# html/semantics/embedded-content/the-img-element/image-loading-lazy-relevant-mutations.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/image-loading-lazy-relevant-mutations.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<head>
  <title>Relevant mutations on deferred loading=lazy images should not trigger
         a request</title>
  <link rel="author" title="Dom Farolino" href="mailto:dom@chromium.org">
  <link rel="help" href="https://html.spec.whatwg.org/multipage/images.html#updating-the-image-data">
  <script src="/resources/testharness.js"></script>
  <script src="/resources/testharnessreport.js"></script>
</head>

<script>
  let below_viewport_1_loaded = false,
      below_viewport_2_loaded = false,
      below_viewport_3_loaded = false

  // For general lazy loading behavior.
  promise_test(() => {
    // When the page loads, start the rest of the tests.
    return new Promise(resolve => {
      window.addEventListener("load", e => {
        const kAssertion = 'image should never load';
        assert_false(below_viewport_1_loaded, `below-viewport-1 ${kAssertion}`);
        assert_false(below_viewport_2_loaded, `below-viewport-2 ${kAssertion}`);
        assert_false(below_viewport_3_loaded, `below-viewport-3 ${kAssertion}`);
        resolve();
      });
    });
  }, "Images are lazyloaded");

  // For `referrerPolicy` attribute mutations.
  promise_test(t => {
    return new Promise((resolve, reject) => {
      const below_viewport_1 = document.querySelector('img#below-viewport-1');
      below_viewport_1.onload = reject;
      below_viewport_1.onerror = reject;
      t.step_timeout(resolve, 1000);

      below_viewport_1.referrerPolicy = 'no-referrer';
    });
  }, "Image referrerPolicy mutation does not cause deferred loading=lazy " +
     "images to be fetched");

  // For `crossOrigin` attribute mutations.
  promise_test(t => {
    return new Promise((resolve, reject) => {
      const below_viewport_2 = document.querySelector('img#below-viewport-2');
      below_viewport_2.onload = reject;
      below_viewport_2.onerror = reject;
      t.step_timeout(resolve, 1000);

      below_viewport_2.crossOrigin = 'anonymous';
    });
  }, "Image crossOrigin mutation does not cause deferred loading=lazy " +
     "images to be fetched");

  // For `src` attribute mutations.
  promise_test(t => {
    return new Promise((resolve, reject) => {
      const below_viewport_3 = document.querySelector('img#below-viewport-3');
      below_viewport_3.onload = reject;
      below_viewport_3.onerror = reject;
      t.step_timeout(resolve, 1000);

      below_viewport_3.src = "resources/image.png?relevant-mutations-change";
    });
  }, "Image src mutation does not cause deferred loading=lazy " +
     "images to be fetched");
</script>

<body>
  <div style="height:1000vh;"></div>
  <img id="below-viewport-1" src="resources/image.png?relevant-mutations-1" loading="lazy"
       onload="below_viewport_1_loaded = true"
       onerror="below_viewport_1_loaded = true">

  <img id="below-viewport-2" src="resources/image.png?relevant-mutations-2" loading="lazy"
       onload="below_viewport_2_loaded = true"
       onerror="below_viewport_2_loaded = true">

  <img id="below-viewport-3" src="resources/image.png?relevant-mutations-3" loading="lazy"
       onload="below_viewport_3_loaded = true"
       onerror="below_viewport_3_loaded = true">
</body>
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
        "byte_end": 2799,
        "byte_start": 2615,
        "col": 3,
        "line": 72
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 2987,
        "byte_start": 2803,
        "col": 3,
        "line": 76
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 3175,
        "byte_start": 2991,
        "col": 3,
        "line": 80
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
  "source_name": "html/semantics/embedded-content/the-img-element/image-loading-lazy-relevant-mutations.html"
}
```
