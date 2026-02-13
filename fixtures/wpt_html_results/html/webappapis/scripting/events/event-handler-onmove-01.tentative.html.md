# html/webappapis/scripting/events/event-handler-onmove-01.tentative.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/scripting/events/event-handler-onmove-01.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Window Object 'onmove'</title>
<link rel="author" title="Sonja Laurila" href="mailto:laurila@google.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#handler-window-onmove">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>

<script>
promise_test(async t => {
  var onMoveCalled = false;
  window.onmove = (event) => { onMoveCalled = true; }
  window.dispatchEvent(new Event('move'));
  assert_true(onMoveCalled, "The onMove handler has been executed.");
}, "Window move event");
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
  "source_name": "html/webappapis/scripting/events/event-handler-onmove-01.tentative.html"
}
```
