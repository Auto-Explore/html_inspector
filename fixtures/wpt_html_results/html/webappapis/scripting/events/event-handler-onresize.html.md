# html/webappapis/scripting/events/event-handler-onresize.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/scripting/events/event-handler-onresize.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>HTMLBodyElement.onresize</title>
<link rel="author" title="His-Name-Is-Joof" href="mailto:jeffrharrison@gmail.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#handler-window-onresize">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<script>
var t = async_test("body.onresize should set the window.onresize handler")
window.onresize = t.step_func(function() {
  assert_unreached("This handler should be overwritten.")
})

var body = document.createElement("body")
body.onresize = t.step_func(function(e) {
  assert_equals(e.currentTarget, window,
                "The event should be fired at the window.")
  t.done()
})
window.dispatchEvent(new Event('resize'));

t = async_test("document.onresize should set the document.onresize handler");
document.onresize = t.step_func(function(e) {
    assert_equals(e.currentTarget, document,
            "The event should be fired at the document")
    t.done()
})
document.dispatchEvent(new Event('resize'));

t = async_test("meta.onresize should set the meta.onresize handler");
var meta = document.createElement("meta")
meta.onresize = t.step_func(function(e) {
    assert_equals(e.currentTarget, meta,
            "The event should be fired at the <meta> object")
    t.done()
})
meta.dispatchEvent(new Event('resize'));
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
  "source_name": "html/webappapis/scripting/events/event-handler-onresize.html"
}
```
