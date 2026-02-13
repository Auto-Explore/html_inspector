# html/syntax/parsing/the-end.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/syntax/parsing/the-end.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<title>The end</title>
<link rel=help href="https://html.spec.whatwg.org/multipage/#the-end">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<script>
async_test(function() {
  document.addEventListener("DOMContentLoaded", this.step_func_done(function(e) {
    assert_equals(e.type, "DOMContentLoaded");
    assert_true(e.bubbles, "bubbles should be true");
    assert_false(e.cancelable, "cancelable should be false");
    assert_equals(e.target, document, "target should be document");
    assert_true(e.isTrusted, "isTrusted should be true");
    assert_class_string(e, "Event");
  }));
}, "DOMContentLoaded");

async_test(function() {
  window.addEventListener("load", this.step_func_done(function(e) {
    assert_equals(e.type, "load");
    assert_false(e.bubbles, "bubbles should be false");
    assert_false(e.cancelable, "cancelable should be false");
    assert_equals(e.target, document, "target should be document");
    assert_true(e.isTrusted, "isTrusted should be true");
    assert_class_string(e, "Event");
  }));
}, "load");

async_test(function() {
  window.addEventListener("pageshow", this.step_func_done(function(e) {
    assert_equals(e.type, "pageshow");

    // https://github.com/whatwg/html/issues/6794
    assert_true(e.bubbles, "bubbles should be true");
    assert_true(e.cancelable, "cancelable should be true");

    assert_equals(e.target, document, "target should be document");
    assert_true(e.isTrusted, "isTrusted should be true");
    assert_class_string(e, "PageTransitionEvent");
  }));
}, "pageshow");

async_test(function() {
  var seen_dcl = false;
  var seen_load = false;
  document.addEventListener("DOMContentLoaded", this.step_func(function() {
    seen_dcl = true;
  }));
  window.addEventListener("load", this.step_func(function() {
    seen_load = true;
    assert_true(seen_dcl, "DOMContentLoaded should be fired before load");
  }));
  window.addEventListener("pageshow", this.step_func_done(function() {
    assert_true(seen_load, "load should be fired before pageshow")
  }));
}, "order");
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
  "source_name": "html/syntax/parsing/the-end.html"
}
```
