# html/semantics/embedded-content/the-img-element/decode/image-decode-with-quick-attach-svg.tentative.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/decode/image-decode-with-quick-attach-svg.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<meta name="timeout" content="long">
<title>SVGImageElement.prototype.decode(), attach to DOM before promise resolves.</title>
<link rel=help href="https://html.spec.whatwg.org/multipage/embedded-content.html#dom-img-decode">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<svg></svg>
<script>
"use strict";

let png = "/images/green.png?image-decode-with-quick-attach-" + Math.random();

promise_test(function() {
  var img = document.createElementNS('http://www.w3.org/2000/svg', 'image');
  img.setAttributeNS('http://www.w3.org/1999/xlink', 'xlink:href', png);
  const promise = img.decode().then(function(arg) {
    assert_equals(arg, undefined);
  });
  // Don't wait for the promise to resolve before attaching the image.
  // The promise should still resolve successfully.
  document.querySelector('svg').appendChild(img);
  return promise;
}, document.title);
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
  "source_name": "html/semantics/embedded-content/the-img-element/decode/image-decode-with-quick-attach-svg.tentative.html"
}
```
