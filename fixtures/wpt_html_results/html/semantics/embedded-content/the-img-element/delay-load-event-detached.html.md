# html/semantics/embedded-content/the-img-element/delay-load-event-detached.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/delay-load-event-detached.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Detached image blocks load</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
var img_loaded = false;

var img = new Image();
img.onload = function() {
  img_loaded = true;
};
img.src = "/images/blue.png?pipe=trickle(d2)";

test(function() {
  assert_false(img_loaded);
}, "setting img.src is async");

async_test(function(t) {
  document.addEventListener("DOMContentLoaded", t.step_func_done(function() {
    assert_false(img_loaded);
  }));
}, "DOMContentLoaded doesn't wait for images");

async_test(function(t) {
  window.addEventListener("load", t.step_func_done(function() {
    assert_true(img_loaded);
  }));
}, "load waits for images");
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
  "source_name": "html/semantics/embedded-content/the-img-element/delay-load-event-detached.html"
}
```
