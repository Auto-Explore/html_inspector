# html/webappapis/scripting/events/event-handler-processing-algorithm-error/worker.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/scripting/events/event-handler-processing-algorithm-error/worker.html",
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
  const worker = new Worker("resources/worker-with-syntax-error.js");
  worker.onerror = t.step_func((...args) => {
    assert_equals(args.length, 1);
    return true;
  });

  const eventWatcher = new EventWatcher(t, worker, "error");
  const promise = eventWatcher.wait_for("error").then(e => {
    assert_equals(e.defaultPrevented, false);
  });

  return promise;
}, "error event is normal (return true does not cancel; one arg) on Worker, with a syntax error in the worker code");

promise_test(t => {
  const worker = new Worker("resources/no-op-worker.js");
  worker.onerror = t.step_func((...args) => {
    assert_equals(args.length, 1);
    return true;
  });

  const eventWatcher = new EventWatcher(t, worker, "error");
  const promise = eventWatcher.wait_for("error").then(e => {
    assert_equals(e.defaultPrevented, false);
  });

  worker.dispatchEvent(new Event("error", { cancelable: true }));

  return promise;
}, "error event is normal (return true does not cancel; one arg) on Worker, with a synthetic Event");

promise_test(t => {
  const worker = new Worker("resources/no-op-worker.js");
  worker.onerror = t.step_func((...args) => {
    assert_equals(args.length, 1);
    return true;
  });

  const eventWatcher = new EventWatcher(t, worker, "error");
  const promise = eventWatcher.wait_for("error").then(e => {
    assert_equals(e.defaultPrevented, false);
  });

  worker.dispatchEvent(new ErrorEvent("error", { cancelable: true }));

  return promise;
}, "error event is normal (return true does not cancel; one arg) on Worker, with a synthetic ErrorEvent");
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
  "source_name": "html/webappapis/scripting/events/event-handler-processing-algorithm-error/worker.html"
}
```
