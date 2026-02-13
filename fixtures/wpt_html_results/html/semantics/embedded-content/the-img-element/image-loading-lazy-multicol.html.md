# html/semantics/embedded-content/the-img-element/image-loading-lazy-multicol.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/image-loading-lazy-multicol.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<head>
  <title>Images with loading='lazy' load when in the viewport</title>
  <link rel="author" title="Chris Harrelson" href="mailto:chrishtr@chromium.org">
  <link rel="help" href="https://github.com/scott-little/lazyload">
  <script src="/resources/testharness.js"></script>
  <script src="/resources/testharnessreport.js"></script>
</head>

<script>
  const t = async_test("Test that images with loading='lazy' under multicol load once they enter the viewport.");

  let has_in_viewport_loaded = false;
  let has_window_loaded = false;

  const in_viewport_img_onload = t.step_func(function() {
    assert_false(has_in_viewport_loaded, "The in_viewport element should load only once.");
    has_in_viewport_loaded = true;
  });

  window.addEventListener("load", t.step_func_done(function() {
    assert_true(has_in_viewport_loaded, "The in_viewport element should have loaded before window.load().");
    assert_false(has_window_loaded, "The window load event should only fire once.");
    has_window_loaded = true;
  }));

</script>

<div class=texty style="column-count: 2; height: 300px">
  <div style="border: 1px solid black">
    <h2 style="column-span: all"></h2>
    <img loading="lazy" src="resources/image.png?loading-lazy-multicol-first" width="160" height="120"
        onload="in_viewport_img_onload()">
  </div>
</div>

  <!--
    This async script loads very slowly in order to ensure that, if the
    below_viewport element has started loading, it has a chance to finish
    loading before window load event fires, so that the test will dependably fail
    in that case instead of potentially passing depending on how long different
    resource fetches take.
  -->
  <script async src="/common/slow.py"></script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 1192,
        "byte_start": 1187,
        "col": 34,
        "line": 31
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 1338,
        "byte_start": 1197,
        "col": 5,
        "line": 32
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
  "source_name": "html/semantics/embedded-content/the-img-element/image-loading-lazy-multicol.html"
}
```
