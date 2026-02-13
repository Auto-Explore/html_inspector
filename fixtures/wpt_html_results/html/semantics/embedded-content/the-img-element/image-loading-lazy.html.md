# html/semantics/embedded-content/the-img-element/image-loading-lazy.html

Counts:
- errors: 0
- warnings: 6
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/image-loading-lazy.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<head>
  <title>Images with loading='lazy' load only when in the viewport</title>
  <link rel="author" title="Dom Farolino" href="mailto:dom@chromium.org">
  <link rel="author" title="Scott Little" href="mailto:sclittle@chromium.org">
  <link rel="help" href="https://github.com/scott-little/lazyload">
  <script src="/resources/testharness.js"></script>
  <script src="/resources/testharnessreport.js"></script>
</head>

<script>
  const in_viewport_test =
    async_test("In-viewport loading=lazy images load immediately but do not " +
               "block the window load event");
  const below_viewport_test =
    async_test("Below-viewport loading=lazy images only load when in the " +
               "viewport and do not block the window load event");
  const below_viewport_data_url_test =
    async_test("Below-viewport data:url images only load when in the " +
               "viewport and do not block the window load event");
  const below_viewport_blob_url_test =
    async_test("Below-viewport blob URL images only load when in the " +
               "viewport and do not block the window load event");

  document.addEventListener('DOMContentLoaded', e => {
    const img = document.querySelector('#below_viewport_blob_url');

    // Blob URL helper.
    // Source: https://bl.ocks.org/nolanlawson/0eac306e4dac2114c752.
    function fixBinary(bin) {
      const length = bin.length;
      const buf = new ArrayBuffer(length);
      const arr = new Uint8Array(buf);
      for (var i = 0; i < length; i++) {
        arr[i] = bin.charCodeAt(i);
      }

      return buf;
    }

    const base64 =
      "R0lGODlhDAAMAKIFAF5LAP/zxAAAANyuAP/gaP///wAAAAAAACH5BAEAAAUALAAAAAAMAAwAAAMlWLPcGjDKFYi9lxKBOaGcF35DhWHamZUW0K4mAbiwWtuf0uxFAgA";
    const binary = fixBinary(atob(base64));
    const blob = new Blob([binary], {type: 'image/png'});
    const url = URL.createObjectURL(blob);
    img.src = url;
  }) // DOMContentLoaded.

  let has_window_load_fired = false;
  let has_in_viewport_loaded = false;

  window.onload = e => {
    has_window_load_fired = true;
  }

  // Helper assertion messages for the below tests.
  const kScrollAssertion = "images only load when scrolled into view";
  const kWindowLoadAssertion = "images do not block the load event";

  const in_viewport_img_onload = in_viewport_test.step_func_done(() => {
    has_in_viewport_loaded = true;
    document.querySelector('#bottom').scrollIntoView();
    assert_true(has_window_load_fired,
                "In-viewport loading=lazy images do not block the window " +
                "load event");
  });

  const below_viewport_img_onload = below_viewport_test.step_func_done(() => {
    assert_true(has_in_viewport_loaded,
                "Below-viewport loading=lazy images only load once loaded " +
                "into the viewport");
    assert_true(has_window_load_fired,
                "Below-viewport loading=lazy images should not block the " +
                "window load event");
  });

  const below_viewport_data_url_img_onload = below_viewport_data_url_test.step_func_done(() => {
    assert_true(has_in_viewport_loaded,
                "Below-viewport loading=lazy data: url images " +
                kScrollAssertion);
    assert_true(has_window_load_fired,
                "Below-viewport loading=lazy data: url images " +
                kWindowLoadAssertion);
  });

  const below_viewport_blob_url_img_onload = below_viewport_blob_url_test.step_func_done(() => {
    assert_true(has_in_viewport_loaded,
                "Below-viewport loading=lazy blob url images " +
                kScrollAssertion);
    assert_true(has_window_load_fired,
                "Below-viewport loading=lazy blob url images " +
                kWindowLoadAssertion);
  });
</script>

<body>
  <!-- |in_viewport| takes 2 seconds to load, so that in browsers that don't
       support lazy loading, |below_viewport| finishes before |in_viewport|, and
       the test will dependably fail without relying on a timeout. -->
  <img id="in_viewport" loading="lazy" src="resources/image.png?image-loading-lazy-first&pipe=trickle(d2)"
       onload="in_viewport_img_onload()">
  <div style="height:1000vh;"></div>
  <img id="below_viewport" loading="lazy" src="resources/image.png?image-loading-lazy-second"
       onload="below_viewport_img_onload()">
  <img id="below_viewport_data_url" loading="lazy"
       src="data:image/png;base64,R0lGODlhDAAMAKIFAF5LAP/zxAAAANyuAP/gaP///wAAAAAAACH5BAEAAAUALAAAAAAMAAwAAAMlWLPcGjDKFYi9lxKBOaGcF35DhWHamZUW0K4mAbiwWtuf0uxFAgA"
       onload="below_viewport_data_url_img_onload()">
  <!-- This image has its `src` set to a blob URL dynamically above -->
  <img id="below_viewport_blob_url" loading="lazy"
       onload="below_viewport_blob_url_img_onload()">
  <div id="bottom"></div>
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
        "byte_end": 4171,
        "byte_start": 4025,
        "col": 3,
        "line": 100
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 4347,
        "byte_start": 4211,
        "col": 3,
        "line": 103
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 4615,
        "byte_start": 4350,
        "col": 3,
        "line": 105
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 4792,
        "byte_start": 4690,
        "col": 3,
        "line": 109
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 4792,
        "byte_start": 4690,
        "col": 3,
        "line": 109
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
  "source_name": "html/semantics/embedded-content/the-img-element/image-loading-lazy.html"
}
```
