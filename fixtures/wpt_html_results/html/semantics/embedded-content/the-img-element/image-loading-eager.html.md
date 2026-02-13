# html/semantics/embedded-content/the-img-element/image-loading-eager.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/image-loading-eager.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<head>
  <title>Images with loading='eager' load immediately regardless of their
         position with respect to the viewport</title>
  <link rel="author" title="Scott Little" href="mailto:sclittle@chromium.org">
  <link rel="help" href="https://github.com/scott-little/lazyload">
  <script src="/resources/testharness.js"></script>
  <script src="/resources/testharnessreport.js"></script>
</head>

<script>
  const t = async_test("Test that images with loading='eager' load " +
                       "immediately regardless of their position with " +
                       "respect to the viewport.");

  let has_in_viewport_loaded = false;
  const in_viewport_img_onload = t.step_func(() => {
    assert_false(has_in_viewport_loaded,
                 "The in_viewport element should load only once.");
    has_in_viewport_loaded = true;
  });

  let has_below_viewport_loaded = false;
  const below_viewport_img_onload = t.step_func(() => {
    assert_false(has_below_viewport_loaded,
                 "The below_viewport element should load only once.");
    has_below_viewport_loaded = true;
  });

  window.addEventListener("load", t.step_func_done(() => {
    assert_true(has_in_viewport_loaded,
                "The in_viewport element should have loaded before window.load().");
    assert_true(has_below_viewport_loaded,
                "The below_viewport element should have loaded before window.load().");
  }));

</script>

<body>
  <img id="in_viewport" src="resources/image.png?in-viewport" loading="eager" onload="in_viewport_img_onload();">
  <div style="height:10000px;"></div>
  <!-- The below_viewport element loads very slowly in order to ensure that the
       window load event is blocked on it. -->
  <img id="below_viewport"
       src="resources/image.png?below-viewport&pipe=trickle(d2)"
       loading="eager" onload="below_viewport_img_onload();">
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
        "byte_end": 1578,
        "byte_start": 1467,
        "col": 3,
        "line": 40
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 1897,
        "byte_start": 1746,
        "col": 3,
        "line": 44
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
  "source_name": "html/semantics/embedded-content/the-img-element/image-loading-eager.html"
}
```
