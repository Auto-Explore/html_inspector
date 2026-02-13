# html/webappapis/scripting/events/uncompiled_event_handler_with_scripting_disabled.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/scripting/events/uncompiled_event_handler_with_scripting_disabled.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<title>Uncompiled event handler check that scripting is enabled</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
  setup({ allow_uncaught_exception: true });
  test(function() {
    var invoked = false;
    window.addEventListener("error", function() {
      invoked = true;
    });

    // Make sure that `this_will_error` will in fact error when it's referenced
    assert_equals(typeof this_will_error, "undefined");
    var dom = (new DOMParser()).parseFromString("<div id=\"has-event-handler\" onclick=\"this_will_error;\"></div>", "text/html");
    var click = new MouseEvent("click");
    dom.getElementById("has-event-handler").dispatchEvent(click);
    assert_equals(invoked, false);
  }, "when scripting is disabled, the handler is never compiled");
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
  "source_name": "html/webappapis/scripting/events/uncompiled_event_handler_with_scripting_disabled.html"
}
```
