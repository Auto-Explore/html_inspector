# html/semantics/embedded-content/the-iframe-element/iframe-loading-lazy-base-url-3.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/iframe-loading-lazy-base-url-3.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<head>
  <title>Deferred iframes with loading='lazy' changed to eager later
         use the document's base URL computed at parse-time</title>
  <link rel="author" title="Oliver Medhurst" href="mailto:omedhurst@mozilla.com">
  <script src="/resources/testharness.js"></script>
  <script src="/resources/testharnessreport.js"></script>
  <script src="../resources/common.js"></script>
  <base href='/html/semantics/embedded-content/the-iframe-element/resources/'>
</head>

<script>
  const below_viewport_iframe = new ElementLoadPromise("below-viewport");

  let has_window_loaded = false;

  async_test(t => {
    // Change the base URL and scroll down to load the deferred iframe.
    window.addEventListener("load", t.step_func(() => {
      const base = document.querySelector('base');
      base.href = '/invalid-url-where-no-subresources-exist/';
      has_window_loaded = true;
      below_viewport_iframe.element().loading = 'eager';
    }));

    below_viewport_iframe.promise.then(
      t.step_func_done(() => {
        assert_true(has_window_loaded,
                    "Below-viewport loading=lazy iframes do not block the " +
                    "window load event");
        assert_true(below_viewport_iframe.element().contentDocument.body.
                    innerHTML.includes("<p>Subframe</p>"),
                    "The loading=lazy iframe's content is accessible upon loading");
    }));

  }, "When a loading=lazy iframe is changed to eager later before loading, it loads relative to the " +
     "document's base URL computed at parse-time.");
</script>

<body>
  <div style="height:1000vh"></div>
  <iframe id="below-viewport" src="subframe.html" loading="lazy"
          width="200px" height="100px" onload="below_viewport_iframe.resolve()">
  </iframe>
</body>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.base.must_come_before_link_or_script",
      "message": "The “base” element must come before any “link” or “script” elements in the document.",
      "severity": "Warning",
      "span": {
        "byte_end": 479,
        "byte_start": 403,
        "col": 3,
        "line": 9
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
  "source_name": "html/semantics/embedded-content/the-iframe-element/iframe-loading-lazy-base-url-3.html"
}
```
