# html/semantics/embedded-content/the-iframe-element/iframe-load-event.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/iframe-load-event.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<title>Test some sanity behavior around iframe load/error events</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<body>
<script>
async_test(function(t) {
  var obj = document.createElement("iframe");
  obj.onload = t.step_func_done(function(e){
    assert_not_equals(obj.contentWindow, null, "The iframe element should represent a nested browsing context.")
    assert_equals(Object.getPrototypeOf(e).constructor, Event, "The load event should use the Event interface.");
    assert_true(e.isTrusted, "The load event should be a trusted event.");
    assert_false(e.cancelable, "The load event should not be a cancelable event.");
    assert_false(e.bubbles, "The load event should not be a bubble event.");
    assert_equals(e.target, obj, "The load event target should be the corresponding object element.");
  });

  obj.onerror = t.unreached_func("The error event should not be fired.");

  var url = URL.createObjectURL(new Blob([""], { type: "text/html" }));

  obj.src = url;
  document.body.appendChild(obj);
}, "load event of blob URL");

async_test(function(t) {
  var obj = document.createElement("iframe");
  obj.onload = t.step_func_done(function(e){
    assert_not_equals(obj.contentWindow, null, "The object element should represent a nested browsing context.")
    assert_equals(Object.getPrototypeOf(e).constructor, Event, "The load event should use the Event interface.");
    assert_true(e.isTrusted, "The load event should be a trusted event.");
    assert_false(e.cancelable, "The load event should not be a cancelable event.");
    assert_false(e.bubbles, "The load event should not be a bubble event.");
    assert_equals(e.target, obj, "The load event target should be the corresponding object element.");
  });

  obj.onerror = t.unreached_func("The error event should not be fired.");

  document.body.appendChild(obj);
}, "load event of initial about:blank");
</script>
</body>
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
  "source_name": "html/semantics/embedded-content/the-iframe-element/iframe-load-event.html"
}
```
