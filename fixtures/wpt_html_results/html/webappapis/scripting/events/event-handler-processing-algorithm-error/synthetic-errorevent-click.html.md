# html/webappapis/scripting/events/event-handler-processing-algorithm-error/synthetic-errorevent-click.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/scripting/events/event-handler-processing-algorithm-error/synthetic-errorevent-click.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Event handlers processing algorithm: click events using ErrorEvent</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<link rel="help" href="https://html.spec.whatwg.org/multipage/#the-event-handler-processing-algorithm">
<link rel="author" title="Domenic Denicola" href="mailto:d@domenic.me">

<div id="log"></div>

<script>
"use strict";
promise_test(t => {
  document.onclick = t.step_func((...args) => {
    assert_equals(args.length, 1);
    return true;
  });

  const eventWatcher = new EventWatcher(t, document, "click");
  const promise = eventWatcher.wait_for("click").then(e => {
    assert_equals(e.defaultPrevented, false);
  });

  document.dispatchEvent(new ErrorEvent("click", { cancelable: true }));

  return promise;
}, "click event is normal (return true does not cancel; one arg) on Document, with a synthetic ErrorEvent");

promise_test(t => {
  window.onclick = t.step_func((...args) => {
    assert_equals(args.length, 1);
    return true;
  });

  const eventWatcher = new EventWatcher(t, window, "click");
  const promise = eventWatcher.wait_for("click").then(e => {
    assert_equals(e.defaultPrevented, false);
  });

  window.dispatchEvent(new ErrorEvent("click", { cancelable: true }));

  return promise;
}, "click event is normal (return true does not cancel; one arg) on Window, with a synthetic ErrorEvent");

promise_test(t => {
  const el = document.createElement("script");
  el.onclick = t.step_func((...args) => {
    assert_equals(args.length, 1);
    return true;
  });

  const eventWatcher = new EventWatcher(t, el, "click");
  const promise = eventWatcher.wait_for("click").then(e => {
    assert_equals(e.defaultPrevented, false);
  });

  el.dispatchEvent(new ErrorEvent("click", { cancelable: true }));

  return promise;
}, "click event is normal (return true does not cancel; one arg) on a script element, with a synthetic ErrorEvent");

promise_test(t => {
  const worker = new Worker("resources/no-op-worker.js");
  worker.onerror = t.step_func((...args) => {
    assert_equals(args.length, 1);
    return true;
  });

  const eventWatcher = new EventWatcher(t, worker, "click");
  const promise = eventWatcher.wait_for("click").then(e => {
    assert_equals(e.defaultPrevented, false);
  });

  worker.dispatchEvent(new ErrorEvent("click", { cancelable: true }));

  return promise;
}, "click event is normal (return true does not cancel; one arg) on Worker, with a synthetic ErrorEvent");
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
  "source_name": "html/webappapis/scripting/events/event-handler-processing-algorithm-error/synthetic-errorevent-click.html"
}
```
