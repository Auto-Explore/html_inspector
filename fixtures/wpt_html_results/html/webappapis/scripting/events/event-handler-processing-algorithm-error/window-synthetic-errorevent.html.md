# html/webappapis/scripting/events/event-handler-processing-algorithm-error/window-synthetic-errorevent.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/scripting/events/event-handler-processing-algorithm-error/window-synthetic-errorevent.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Event handlers processing algorithm: error events</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<link rel="help" href="https://html.spec.whatwg.org/multipage/#the-event-handler-processing-algorithm">
<link rel="author" title="Domenic Denicola" href="mailto:d@domenic.me">

<div id="log"></div>

<script>
"use strict";
setup({ allow_uncaught_exception: true });

promise_test(t => {
  window.onerror = t.step_func((...args) => {
    assert_greater_than(args.length, 1);
    return true;
  });

  const eventWatcher = new EventWatcher(t, window, "error");
  const promise = eventWatcher.wait_for("error").then(e => {
    assert_equals(e.defaultPrevented, true);
  });

  window.dispatchEvent(new ErrorEvent("error", { cancelable: true }));

  return promise;
}, "error event is weird (return true cancels; many args) on Window, with a synthetic ErrorEvent");

promise_test(t => {
  const theError = { the: "error object" };

  window.onerror = t.step_func(function (message, filename, lineno, colno, error) {
    assert_equals(arguments.length, 5, "There must be exactly 5 arguments");
    assert_equals(message, "message");
    assert_equals(filename, "filename");
    assert_equals(lineno, 1);
    assert_equals(colno, 2);
    assert_equals(error, theError);
    return true;
  });

  const eventWatcher = new EventWatcher(t, window, "error");
  const promise = eventWatcher.wait_for("error");

  window.dispatchEvent(new ErrorEvent("error", {
    message: "message",
    filename: "filename",
    lineno: 1,
    colno: 2,
    error: theError
  }));

  return promise;
}, "error event has the right 5 args on Window, with a synthetic ErrorEvent");
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
  "source_name": "html/webappapis/scripting/events/event-handler-processing-algorithm-error/window-synthetic-errorevent.html"
}
```
