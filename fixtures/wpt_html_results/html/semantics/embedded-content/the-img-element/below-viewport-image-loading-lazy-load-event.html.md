# html/semantics/embedded-content/the-img-element/below-viewport-image-loading-lazy-load-event.html

Counts:
- errors: 3
- warnings: 5
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/below-viewport-image-loading-lazy-load-event.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<head>
  <title>Below-viewport loading=lazy images do not block the window load event
         when scrolled into viewport</title>
  <link rel="author" title="Rob Buis" href="mailto:rbuis@igalia.com">
  <link rel="author" title="Dom Farolino" href="mailto:dom@chromium.org">
  <script src="/resources/testharness.js"></script>
  <script src="/resources/testharnessreport.js"></script>
  <script src="../resources/common.js"></script>
</head>

<body>
  <!-- When this image loads, we will scroll the below-viewport loading=lazy
       images into the viewport. This happens before the window load event is
       fired -->
  <img id="scroll_trigger"
       src="resources/image.png?scroll-trigger&pipe=trickle(d1)"
       onload="scroll_trigger_img.resolve();" onerror="scroll_trigger_img.reject();">
  <!-- This image blocks the window load event for 2 seconds -->
  <img src="resources/image.png?window-load-blocking&pipe=trickle(d2)">

  <div style="height:1000vh"></div>
  <!-- These images must load because they intersect the viewport, but they must
       not block the window load event, because they are loading=lazy -->
  <img id="visible"
       src="resources/image.png?visible&pipe=trickle(d3)" loading="lazy"
       onload="visible_img.resolve();" onerror="visible_img.reject();">
  <img id="visibility_hidden" style="visibility:hidden;"
       src="resources/image.png?visibility_hidden&pipe=trickle(d3)" loading="lazy"
       onload="visibility_hidden_img.resolve();" onerror="visibility_hidden_img.reject();">
</body>

<script>
  const scroll_trigger_img = new ElementLoadPromise("visible");
  const visible_img = new ElementLoadPromise("visible");
  const visibility_hidden_img = new ElementLoadPromise("visibility_hidden");

  async_test(t => {
    let has_window_loaded = false;

    scroll_trigger_img.promise
      .then(t.step_func(() => {
        assert_false(has_window_loaded,
                     "The scroll_trigger image should load before the window " +
                     "load event fires");
        visibility_hidden_img.element().scrollIntoView();
    }))
    .catch(t.unreached_func("The scroll_trigger image should load"));

    window.addEventListener("load", t.step_func(() => {
      has_window_loaded = true;
    }));

    Promise.all([visible_img.promise, visibility_hidden_img.promise])
      .then(t.step_func_done(() => {
        assert_true(has_window_loaded,
                    "The window load event should fire before the " +
                    "below-viewport loading=lazy images load");
        assert_true(visible_img.element().complete,
                    "The below-viewport loading=lazy visible image is complete");
        assert_true(visibility_hidden_img.element().complete,
                    "The below-viewport loading=lazy visibility:hidden image is complete");
      }))
      .catch(t.unreached_func("The images should load successfully"));

  }, "Below-viewport loading=lazy images do not block the window load event when " +
     "scrolled into viewport");
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
        "byte_end": 815,
        "byte_start": 640,
        "col": 3,
        "line": 16
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 952,
        "byte_start": 883,
        "col": 3,
        "line": 20
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 1309,
        "byte_start": 1147,
        "col": 3,
        "line": 25
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 1541,
        "byte_start": 1312,
        "col": 3,
        "line": 28
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_start_tag",
      "message": "Stray start tag “script”.",
      "severity": "Error",
      "span": {
        "byte_end": 1559,
        "byte_start": 1551,
        "col": 1,
        "line": 33
      }
    },
    {
      "category": "Html",
      "code": "html.parser.nonspace_after_body",
      "message": "Non-space character after body.",
      "severity": "Error",
      "span": {
        "byte_end": 3043,
        "byte_start": 1559,
        "col": 9,
        "line": 33
      }
    },
    {
      "category": "Html",
      "code": "html.parser.end_tag_after_body_closed",
      "message": "Saw an end tag after “body” had been closed.",
      "severity": "Error",
      "span": {
        "byte_end": 3052,
        "byte_start": 3043,
        "col": 1,
        "line": 68
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
  "source_name": "html/semantics/embedded-content/the-img-element/below-viewport-image-loading-lazy-load-event.html"
}
```
