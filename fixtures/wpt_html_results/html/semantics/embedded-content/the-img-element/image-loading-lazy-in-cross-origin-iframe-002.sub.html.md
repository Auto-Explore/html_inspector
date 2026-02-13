# html/semantics/embedded-content/the-img-element/image-loading-lazy-in-cross-origin-iframe-002.sub.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/image-loading-lazy-in-cross-origin-iframe-002.sub.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<head>
<title>A loading=lazy image in a below-viewport cross-origin iframe loads only
       when the cross-origin iframe is scrolled into view</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/urls-and-fetching.html#lazy-loading-attributes">
<link rel="author" title="Dom Farolino" href="mailto:dom@chromium.org">
<link rel="author" title="Rob Buis" href="mailto:rbuis@igalia.com">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/common/get-host-info.sub.js"></script>
</head>

<div style="height:1000vh;"></div>
<iframe id="iframe" width="500px" height="500px"></iframe>

<script>
promise_test(t => {
  iframe.src =
    get_host_info().HTTP_NOTSAMESITE_ORIGIN +
    new URL("resources/", self.location).pathname +
    "image-loading-lazy-in-viewport.html";

  // Wait for the frame to report that its window load event fired.
  return new Promise(resolve => {
    window.addEventListener("message",
                            event => resolve(event.data), {once: true});
  }).then(iframe_message => {
    assert_equals(iframe_message, "window_loaded",
                  "The loading=lazy image should not block the iframe's load " +
                  "event");

    // Scroll the iframe into view, which also puts the image into view.
    iframe.scrollIntoView();

    return new Promise(resolve => {
      window.addEventListener("message", event => resolve(event.data));
    });
  }).then(iframe_message => {
    assert_equals(iframe_message, "image_loaded",
                  "The below-viewport loading=lazy image should load only " +
                  "once scrolled into the viewport");

  }); // new Promise().

}); // promise_test().
</script>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/embedded-content/the-img-element/image-loading-lazy-in-cross-origin-iframe-002.sub.html"
}
```
