# html/semantics/embedded-content/the-img-element/image-loading-lazy-negative-margin.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/image-loading-lazy-negative-margin.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<head>
  <title>Images with loading='lazy' defers images in a hidden area as a result
         of negative margins</title>
  <link rel="author" title="Dom Farolino" href="mailto:dom@chromium.org">
  <link rel="help" href="https://html.spec.whatwg.org/multipage/urls-and-fetching.html#lazy-loading-attributes">
  <script src="/resources/testharness.js"></script>
  <script src="/resources/testharnessreport.js"></script>
</head>

<body>
  <script>
    window.negative_margin_test =
      async_test("A loading=lazy image that is pulled into an `overflow: hidden` " +
                 "area by a negative margin will not load because " +
                 "IntersectionObserver sees it as non-intersecting");

    // If the `negative_margin` image in the DOM loads, the test should fail
    // immediately.
    window.negative_margin_onload =
      negative_margin_test.step_func_done(
        negative_margin_test.unreached_func("The image with a negative margin " +
                                            "should never load"));
  </script>

  <div style="width: 200px; height: 200px; overflow: hidden;">
    <img id="negative_margin" width="5px"; style="margin-left: -10000vw;"
         loading="lazy" src="resources/image.png?loading-lazy-negative-margin"
         onload="window.negative_margin_onload()">
  </div>

  <script>
    const intersection_observer_promise = new Promise(resolve => {
      function io_callback(entries) {
        assert_equals(entries.length, 1);
        resolve(entries[0].isIntersecting);
      }

      const options = {
        root: document,
        rootMargin: '0px',
        threshold: 1.0,
      }

      const observer = new IntersectionObserver(io_callback, options);
      observer.observe(document.querySelector('#negative_margin'));
    });

    const timeout_promise = new Promise(resolve => {
      window.negative_margin_test.step_timeout(resolve, 500);
    });

    Promise.all([intersection_observer_promise, timeout_promise]).then(
      window.negative_margin_test.step_func_done(values => {
        assert_equals(values.length, 2);
        assert_equals(values[0], false, "The IntersectionObserver sees that " +
                                        "the image does not intersect the " +
                                        "viewport");
      }));
  </script>
</body>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.img.width.bad_value",
      "message": "Bad value “5px” for attribute “width” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1327,
        "byte_start": 1128,
        "col": 5,
        "line": 27
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 1327,
        "byte_start": 1128,
        "col": 5,
        "line": 27
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
  "source_name": "html/semantics/embedded-content/the-img-element/image-loading-lazy-negative-margin.html"
}
```
