# html/semantics/embedded-content/the-img-element/decode/image-decode-iframe.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/decode/image-decode-iframe.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<meta name="timeout" content="long">
<title>HTMLImageElement.prototype.decode(), iframe tests.</title>
<link rel="author" title="Vladimir Levin" href="mailto:vmpstr@chromium.org">
<link rel="help" href="https://html.spec.whatwg.org/multipage/embedded-content.html#dom-img-decode">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<iframe id="frame_loaded" srcdoc="iframe"></iframe>
<iframe id="frame_notloaded" srcdoc="iframe"></iframe>
<iframe id="frame_notloaded2" srcdoc="iframe"></iframe>

<script>
"use strict";

promise_test(function() {
  return new Promise(function(resolve, reject) {
    var frame = document.getElementById("frame_loaded");
    var img = frame.contentDocument.createElement("img");
    img.src = "/images/green.png";
    img.onload = function() {
      // At this point the frame which created the img is removed, so decode() should fail.
      frame.parentNode.removeChild(frame);
      img.decode().then(function() {
        assert_unreached("Unexpected success");
      }, function() {
        resolve();
      });
    };
  });
}, document.title + " Decode from removed iframe fails (loaded img)");

promise_test(function(t) {
  var frame = document.getElementById("frame_notloaded");
  var img = frame.contentDocument.createElement("img");
  img.src = "/images/green.png";
  frame.parentNode.removeChild(frame);
  var promise = img.decode();
  return promise_rejects_dom(t, "EncodingError", promise);
}, document.title + " Decode from removed iframe fails (img not loaded)");

promise_test(function(t) {
  var frame = document.getElementById("frame_notloaded2");
  var img = frame.contentDocument.createElement("img");
  img.src = "/images/green.png";
  // First request a promise, then remove the iframe.
  var promise = img.decode();
  frame.parentNode.removeChild(frame);
  return promise_rejects_dom(t, "EncodingError", promise);
}, document.title + " Decode from iframe, later removed, fails (img not loaded)");

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
  "source_name": "html/semantics/embedded-content/the-img-element/decode/image-decode-iframe.html"
}
```
