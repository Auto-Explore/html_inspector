# html/semantics/embedded-content/the-img-element/delay-load-event.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/delay-load-event.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Inline image element blocks load</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
var img_loaded = false;
</script>
<img src="/images/blue.png?pipe=trickle(d2)" onload="img_loaded = true;">
<script>
test(function() {
  assert_false(img_loaded);
}, "script execution doesn't wait for the image to load");

async_test(function(t) {
  document.addEventListener("DOMContentLoaded", t.step_func_done(function() {
    assert_false(img_loaded);
  }));
}, "DOMContentLoaded doesn't wait for images");

async_test(function(t) {
  window.addEventListener("load", t.step_func_done(function() {
    assert_true(img_loaded);
  }));
}, "Image element delays window's load event");
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
        "byte_end": 309,
        "byte_start": 236,
        "col": 1,
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
  "source_name": "html/semantics/embedded-content/the-img-element/delay-load-event.html"
}
```
