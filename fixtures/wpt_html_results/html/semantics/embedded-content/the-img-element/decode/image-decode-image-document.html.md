# html/semantics/embedded-content/the-img-element/decode/image-decode-image-document.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/decode/image-decode-image-document.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>HTMLImageElement.prototype.decode(), image document tests.</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/embedded-content.html#dom-img-decode">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<iframe id="frame_imgdoc" src="about:blank"></iframe>
<script>
"use strict";

promise_test(function() {
  return new Promise(function(resolve) {
    var frame = document.getElementById("frame_imgdoc");
    // Load an image in the iframe and then replace that.
    frame.src = "/images/red.png";
    frame.onload = function() {
      let img = frame.contentDocument.body.firstElementChild;
      img.src = "/images/green.png";
      img.decode().then(function() {
        resolve();
      });
    };
  });
}, document.title + " Decode from iframe with image document, succeeds (img not loaded)");
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
  "source_name": "html/semantics/embedded-content/the-img-element/decode/image-decode-image-document.html"
}
```
