# html/semantics/embedded-content/the-img-element/image-loading-lazy-base-url-2.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/image-loading-lazy-base-url-2.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<head>
  <title>Deferred loading=lazy images load relative to the document's base URL
         at parse-time</title>
  <link rel="author" title="Dom Farolino" href="mailto:dom@chromium.org">
  <link rel="author" title="Raj T" href="mailto:rajendrant@chromium.org">
  <script src="/resources/testharness.js"></script>
  <script src="/resources/testharnessreport.js"></script>
  <script src="../resources/common.js"></script>
</head>

<script>
  const below_viewport_img = new ElementLoadPromise("below-viewport");

  let has_window_loaded = false;

  async_test(t => {
    // Change the document's base URL to a bogus one, and scroll the
    // below-viewport img into view. When it loads, it should load relative
    // to the old base URL computed at parse-time.
    window.addEventListener("load", t.step_func(() => {
      window.history.pushState(2, document.title,
                               '/invalid-url-where-no-subresources-exist/')
      has_window_loaded = true;
      below_viewport_img.element().scrollIntoView();
    }));

    below_viewport_img.promise.then(t.step_func_done(() => {
      assert_true(has_window_loaded,
                  "Below-viewport loading=lazy images do not block the " +
                  "window load event");
    }));

    below_viewport_img.promise.catch(
      t.unreached_func("The image request should not load relative to the " +
                       "current (incorrect) base URL.")
    );
  }, "When a loading=lazy image is loaded, it loads relative to the " +
     "document's base URL computed at parse-time.");
</script>

<body>
  <div style="height:1000vh;"></div>
  <script>
    // Change the document's base URL so that the img request parses relative
    // to it when it sets up the request at parse-time.
    window.history.pushState(1, document.title, 'resources/')
  </script>
  <img id="below-viewport" src="image.png?base-url-2" loading="lazy"
       onload="below_viewport_img.resolve()"
       onerror="below_viewport_img.reject()">
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
        "byte_end": 2017,
        "byte_start": 1860,
        "col": 3,
        "line": 49
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
  "source_name": "html/semantics/embedded-content/the-img-element/image-loading-lazy-base-url-2.html"
}
```
