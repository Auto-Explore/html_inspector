# html/semantics/embedded-content/the-img-element/decode/image-decode-picture.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/decode/image-decode-picture.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<meta name="timeout" content="long">
<title>HTMLImageElement.prototype.decode(), picture tests.</title>
<link rel="author" title="Vladimir Levin" href="mailto:vmpstr@chromium.org">
<link rel=help href="https://html.spec.whatwg.org/multipage/embedded-content.html#dom-img-decode">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<picture>
<source srcset="/images/green.png">
<source srcset="/images/blue.png">
<img id="testimg">
</picture>

<script>
"use strict";

promise_test(function() {
  var picture = document.createElement("picture");
  var source = document.createElement("source");
  var img = document.createElement("img");

  picture.appendChild(source);
  picture.appendChild(img);

  source.srcset = "/images/green.png";

  return img.decode().then(function(arg) {
    assert_equals(arg, undefined);
  });
}, document.title + " Image with PNG source decodes with undefined.");

promise_test(function() {
  var img = document.getElementById("testimg");
  return img.decode().then(function(arg) {
    assert_equals(arg, undefined);
  });
}, document.title + " Image with multiple sources decodes with undefined.");

promise_test(function() {
  var picture = document.createElement("picture");
  var source = document.createElement("source");
  var img = document.createElement("img");

  picture.appendChild(source);
  picture.appendChild(img);

  source.srcset = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAIAAAACCAIA" +
                  "AAD91JpzAAAACXBIWXMAAAsTAAALEwEAmpwYAAAAB3RJTUUH4QUSEioKsy" +
                  "AgywAAABl0RVh0Q29tbWVudABDcmVhdGVkIHdpdGggR0lNUFeBDhcAAAAW" +
                  "SURBVAjXY9y3bx8DAwPL58+fGRgYACktBRltLfebAAAAAElFTkSuQmCC";

  return img.decode().then(function(arg) {
    assert_equals(arg, undefined);
  });
}, document.title + " Image with PNG data URL source decodes with undefined.");

promise_test(function() {
  var picture = document.createElement("picture");
  var source = document.createElement("source");
  var img = document.createElement("img");

  picture.appendChild(source);
  picture.appendChild(img);

  source.srcset = "/images/green.svg";

  return img.decode().then(function(arg) {
    assert_equals(arg, undefined);
  });
}, document.title + " Image with SVG source decodes with undefined.");

promise_test(function(t) {
  var picture = document.createElement("picture");
  var source = document.createElement("source");
  var img = document.createElement("img");

  picture.appendChild(source);
  picture.appendChild(img);

  source.srcset = "/non/existent/path.png";

  var promise = img.decode();
  return promise_rejects_dom(t, "EncodingError", promise);
}, document.title + " Non-existent source fails decode.");

promise_test(function(t) {
  var picture = document.createElement("picture");
  var source = document.createElement("source");
  var img = document.createElement("img");

  picture.appendChild(source);
  picture.appendChild(img);

  source.srcset = "data:image/png;base64,iVBO00PDR0BADBEEF00KGg";

  var promise = img.decode();
  return promise_rejects_dom(t, "EncodingError", promise);
}, document.title + " Corrupt image in src fails decode.");

promise_test(function(t) {
  var picture = document.createElement("picture");
  var source = document.createElement("source");
  var img = document.createElement("img");

  picture.appendChild(source);
  picture.appendChild(img);

  var promise = img.decode();
  return promise_rejects_dom(t, "EncodingError", promise);
}, document.title + " Image without srcset fails decode.");

promise_test(function() {
  var picture = document.createElement("picture");
  var source = document.createElement("source");
  var img = document.createElement("img");

  picture.appendChild(source);
  picture.appendChild(img);

  source.srcset = "/images/green.png";

  var first_promise = img.decode();
  var second_promise = img.decode();
  assert_not_equals(first_promise, second_promise);
  return Promise.all([
    first_promise,
    second_promise
  ]);
}, document.title + " Multiple decodes for images with src succeed.");

</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.picture.source.always_matching.disallowed",
      "message": "A “source” element that has a following sibling “source” element or “img” element with a “srcset” attribute must have a “media” attribute and/or “type” attribute.",
      "severity": "Warning",
      "span": {
        "byte_end": 469,
        "byte_start": 434,
        "col": 1,
        "line": 11
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 523,
        "byte_start": 505,
        "col": 1,
        "line": 13
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 523,
        "byte_start": 505,
        "col": 1,
        "line": 13
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
  "source_name": "html/semantics/embedded-content/the-img-element/decode/image-decode-picture.html"
}
```
