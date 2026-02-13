# html/semantics/embedded-content/the-iframe-element/iframe-loading-lazy-load-event.html

Counts:
- errors: 3
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/iframe-loading-lazy-load-event.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<head>
  <title>In-viewport loading=lazy iframes do not block the window load event</title>
  <link rel="author" title="Dom Farolino" href="mailto:dom@chromium.org">
  <script src="/resources/testharness.js"></script>
  <script src="/resources/testharnessreport.js"></script>
  <script src="../resources/common.js"></script>
</head>

<body>
  <!-- This image blocks the window load event for 1 second -->
  <img src="/common/square.png?pipe=trickle(d1)">

  <!-- These iframes must load because they intersect the viewport, but they must
       not block the window load event, because they are loading=lazy -->
  <iframe id="visible" loading="lazy"
       src="resources/subframe.html?visible&pipe=trickle(d3)"></iframe>
  <iframe id="visibility_hidden" style="visibility:hidden;" loading="lazy"
       src="resources/subframe.html?visibility_hidden&pipe=trickle(d3)"></iframe>
</body>

<script>
  const visible_iframe = document.querySelector('#visible');
  const hidden_iframe = document.querySelector('#visibility_hidden');

  const visible_iframe_t =
    async_test('In-viewport loading=lazy iframe does not block the load event');

  const hidden_iframe_t =
    async_test('In-viewport loading=lazy visibility:hidden iframe does not ' +
               'block the load event');

  let has_window_loaded = false;
  window.addEventListener("load", () => {
    has_window_loaded = true;
  });

  visible_iframe.onload = visible_iframe_t.step_func_done(() => {
    assert_true(has_window_loaded,
                "The visible iframe should not block the load event");
  });

  hidden_iframe.onload = hidden_iframe_t.step_func_done(() => {
    assert_true(has_window_loaded,
                "The hidden iframe should not block the load event");
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
        "byte_end": 470,
        "byte_start": 423,
        "col": 3,
        "line": 12
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_start_tag",
      "message": "Stray start tag “script”.",
      "severity": "Error",
      "span": {
        "byte_end": 912,
        "byte_start": 904,
        "col": 1,
        "line": 22
      }
    },
    {
      "category": "Html",
      "code": "html.parser.nonspace_after_body",
      "message": "Non-space character after body.",
      "severity": "Error",
      "span": {
        "byte_end": 1766,
        "byte_start": 912,
        "col": 9,
        "line": 22
      }
    },
    {
      "category": "Html",
      "code": "html.parser.end_tag_after_body_closed",
      "message": "Saw an end tag after “body” had been closed.",
      "severity": "Error",
      "span": {
        "byte_end": 1775,
        "byte_start": 1766,
        "col": 1,
        "line": 47
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
  "source_name": "html/semantics/embedded-content/the-iframe-element/iframe-loading-lazy-load-event.html"
}
```
