# html/webappapis/scripting/events/event-handler-processing-algorithm-error/script-element.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/scripting/events/event-handler-processing-algorithm-error/script-element.html",
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

promise_test(t => {
  const script = document.createElement("script");
  script.onerror = t.step_func((...args) => {
    assert_equals(args.length, 1);
    return true;
  });

  const eventWatcher = new EventWatcher(t, script, "error");
  const promise = eventWatcher.wait_for("error").then(e => {
    assert_equals(e.constructor, Event); // not ErrorEvent
    assert_equals(e.defaultPrevented, false);
  });

  script.src = "404.js";
  document.body.appendChild(script);

  return promise;
}, "error event behaves normally (return true does not cancel; one arg) on a script element, with a 404 error");

promise_test(t => {
  const script = document.createElement("script");
  script.onerror = t.step_func((...args) => {
    assert_equals(args.length, 1);
    return true;
  });

  const eventWatcher = new EventWatcher(t, script, "error");
  const promise = eventWatcher.wait_for("error").then(e => {
    assert_equals(e.defaultPrevented, false);
  });

  script.dispatchEvent(new Event("error", { cancelable: true }));

  return promise;
}, "error event behaves normally (return true does not cancel; one arg) on a script element, with a synthetic Event");

promise_test(t => {
  const script = document.createElement("script");
  script.onerror = t.step_func((...args) => {
    assert_equals(args.length, 1);
    return true;
  });

  const eventWatcher = new EventWatcher(t, script, "error");
  const promise = eventWatcher.wait_for("error").then(e => {
    assert_equals(e.defaultPrevented, false);
  });

  script.dispatchEvent(new ErrorEvent("error", { cancelable: true }));

  return promise;
}, "error event behaves normally (return true does not cancel; one arg) on a script element, with a synthetic ErrorEvent");

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
  "source_name": "html/webappapis/scripting/events/event-handler-processing-algorithm-error/script-element.html"
}
```
