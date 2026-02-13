# html/semantics/embedded-content/the-img-element/image-loading-lazy-in-viewport-dynamic.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/image-loading-lazy-in-viewport-dynamic.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<head>
  <title>In viewport images with loading='lazy' and changed to loading='eager'
         do not block the window load event</title>
  <link rel="author" title="Rob Buis" href="mailto:rbuis@igalia.com">
  <link rel="help" href="https://github.com/scott-little/lazyload">
  <script src="/resources/testharness.js"></script>
  <script src="/resources/testharnessreport.js"></script>
</head>

<script>
  const t = async_test("Test that in viewport images with loading='lazy' and " +
                       "changed to loading='eager' do not block the window " +
                       "load event.");

  let has_in_viewport_loaded = false;
  let has_window_loaded = false;

  const in_viewport_img_onload = t.step_func_done(function() {
    assert_false(has_in_viewport_loaded,
               "The in_viewport element should load only once.");
    assert_true(has_window_loaded,
               "The window load event should fire before in_viewport image loads.");
    has_in_viewport_loaded = true;
  });

  window.addEventListener("load", t.step_func(function() {
    assert_false(has_window_loaded,
                 "The window load event should only fire once.");
    has_window_loaded = true;
  }));
</script>

<body>
  <img id="in_viewport" src="resources/image.png?in-viewport-dynamic&pipe=trickle(d2)"
       loading="lazy" onload="in_viewport_img_onload();">
  <script>
    document.getElementById("in_viewport").loading = 'eager';
  </script>
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
        "byte_end": 1384,
        "byte_start": 1242,
        "col": 3,
        "line": 35
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
  "source_name": "html/semantics/embedded-content/the-img-element/image-loading-lazy-in-viewport-dynamic.html"
}
```
