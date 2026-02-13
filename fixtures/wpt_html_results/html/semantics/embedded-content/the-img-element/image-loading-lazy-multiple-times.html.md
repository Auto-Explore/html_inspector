# html/semantics/embedded-content/the-img-element/image-loading-lazy-multiple-times.html

Counts:
- errors: 1
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/image-loading-lazy-multiple-times.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<head>
  <title>Images with loading='lazy' can be lazy loaded multiple times</title>
  <link rel="author" title="Dom Farolino" href="mailto:dom@chromium.org">
  <link rel="help" href="https://html.spec.whatwg.org/multipage/urls-and-fetching.html#lazy-loading-attributes">
  <script src="/resources/testharness.js"></script>
  <script src="/resources/testharnessreport.js"></script>
</head>

<body>
  <!-- This is used to represent the top of the viewport, so we can scroll the
       below-viewport image out-of-view later in the test -->
  <div id="top_div"></div>
  <div style="height:1000vh;"></div>
  <img id="below_viewport" loading="lazy" src="resources/image.png?image-loading-lazy-multiple-times-first">

<script>
  const t = async_test("Images with loading='lazy' can be lazy loaded multiple times");
  const image = document.querySelector('#below_viewport');
  const top_div = document.querySelector('#top_div');

  let has_window_load_fired = false;

  // This should be triggered first.
  window.addEventListener('load', t.step_func(() => {
    has_window_load_fired = true;
    // Scroll the loading=lazy below-viewport image into view, so that it loads.
    image.scrollIntoView();
  }));

  image.onload = t.step_func(() => {
    assert_true(has_window_load_fired,
                "The loading=lazy below-viewport image should not block the " +
                "window load event");
    changeImageSourceAndScrollToTop();
  });

  function changeImageSourceAndScrollToTop() {
    top_div.scrollIntoView();

    // Allow some time for scroll back to top, since we don't
    // want the image to still be in the viewport and trigger a
    // load due to the scroll being slow.
    t.step_timeout(() => {
      // Lazily load a "different" image.
      image.src = 'resources/image.png?image-loading-lazy-multiple-times-second';
      image.onload =
        t.unreached_func("The loading=lazy below-viewport image should lazily " +
                         "load its second image, and not load it eagerly when " +
                         "the `src` attribute is changed");

      // In 1s, scroll the image *back* into view, and record that it loads
      // successfully.
      t.step_timeout(() => {
        image.onload = t.step_func_done();
        image.scrollIntoView();
      }, 1000);
    }, 500);
  }
</script>
</body>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.start_tag_without_doctype",
      "message": "Start tag seen without seeing a doctype first. Expected “<!DOCTYPE html>”.",
      "severity": "Error",
      "span": {
        "byte_end": 6,
        "byte_start": 0,
        "col": 1,
        "line": 1
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 711,
        "byte_start": 605,
        "col": 3,
        "line": 14
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
  "source_name": "html/semantics/embedded-content/the-img-element/image-loading-lazy-multiple-times.html"
}
```
