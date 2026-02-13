# html/semantics/embedded-content/the-iframe-element/iframe-loading-lazy-base-url-2.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/iframe-loading-lazy-base-url-2.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<head>
  <title>Deferred loading=lazy iframes load relative to the document's base URL
         at parse-time</title>
  <link rel="author" title="Dom Farolino" href="mailto:dom@chromium.org">
  <link rel="author" title="Raj T" href="mailto:rajendrant@chromium.org">
  <script src="/resources/testharness.js"></script>
  <script src="/resources/testharnessreport.js"></script>
  <script src="../resources/common.js"></script>
</head>

<script>
  const below_viewport_iframe = new ElementLoadPromise("below-viewport");

  let has_window_loaded = false;

  async_test(t => {
    // Change the document's base URL to a bogus one, and scroll the
    // below-viewport iframe into view. When it loads, it should load relative
    // to the old base URL computed at parse-time.
    window.addEventListener("load", t.step_func(() => {
      window.history.pushState(2, document.title,
                               '/invalid-url-where-no-subresources-exist/')
      has_window_loaded = true;
      below_viewport_iframe.element().scrollIntoView();
    }));

    below_viewport_iframe.promise.then(t.step_func_done(() => {
      assert_true(has_window_loaded,
            "Below-viewport loading=lazy iframes do not block the " +
            "window load event");
      assert_true(below_viewport_iframe.element()
                    .contentDocument.body.innerHTML.includes("<p>Subframe</p>"));
    }));

  }, "When a loading=lazy iframe is loaded, it loads relative to the " +
     "document's base URL computed at parse-time.");
</script>

<body>
  <div style="height:1000vh;"></div>
  <script>
    // Change the document's base URL so that the iframe request parses relative
    // to it when it sets up the request at parse-time.
    window.history.pushState(1, document.title, 'resources/')
  </script>
  <iframe id="below-viewport" src="subframe.html" loading="lazy" width="200px"
          height="100px" onload="below_viewport_iframe.resolve()"</iframe>
</body>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.tokenizer.lt_expecting_attr_name",
      "message": "Saw “<” when expecting an attribute name. Probable cause: Missing “>” immediately before.",
      "severity": "Warning",
      "span": {
        "byte_end": 1970,
        "byte_start": 1819,
        "col": 3,
        "line": 47
      }
    },
    {
      "category": "Html",
      "code": "html.iframe.text.disallowed",
      "message": "Text not allowed in “iframe” in this context.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/embedded-content/the-iframe-element/iframe-loading-lazy-base-url-2.html"
}
```
