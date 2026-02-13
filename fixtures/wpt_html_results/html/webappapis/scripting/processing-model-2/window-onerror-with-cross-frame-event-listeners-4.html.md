# html/webappapis/scripting/processing-model-2/window-onerror-with-cross-frame-event-listeners-4.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/scripting/processing-model-2/window-onerror-with-cross-frame-event-listeners-4.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<title>
  When a listener from window A is added to an event target in window A via the
  addEventListener function from window B, errors in that listener should be
  reported to window A.
</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<iframe></iframe>
<iframe></iframe>
<script>
test(function() {
  var f = new frames[1].Function("thereIsNoSuchCallable()");
  frames[0].document.addEventListener.call(frames[1].document, "myevent", f);
  var frame0ErrorFired = false;
  var frame1ErrorFired = false;
  var ourErrorFired = false;
  frames[0].addEventListener("error", function() {
    frame0ErrorFired = true;
  });
  frames[1].addEventListener("error", function() {
    frame1ErrorFired = true;
  });
  addEventListener("error", function() {
    ourErrorFired = true;
  });
  frames[1].document.dispatchEvent(new Event("myevent"));
  assert_false(frame0ErrorFired);
  assert_true(frame1ErrorFired);
  assert_false(ourErrorFired);
}, "The error event from an event listener should fire on that listener's global");
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
  "source_name": "html/webappapis/scripting/processing-model-2/window-onerror-with-cross-frame-event-listeners-4.html"
}
```
