# html/webappapis/scripting/events/body-onload.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/scripting/events/body-onload.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>HTMLBodyElement.onload</title>
<link rel="author" title="Boris Zbarsky" href="mailto:bzbarsky@mit.edu">
<link rel="author" title="Ms2ger" href="mailto:ms2ger@gmail.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#handler-window-onload">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<script>
var t = async_test("body.onload should set the window.onload handler")
window.onload = t.step_func(function() {
  assert_unreached("This handler should be overwritten.")
})
var b = document.createElement("body")
b.onload = t.step_func(function(e) {
  assert_equals(e.currentTarget, window,
                "The event should be fired at the window.")
  t.done()
})
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
  "source_name": "html/webappapis/scripting/events/body-onload.html"
}
```
