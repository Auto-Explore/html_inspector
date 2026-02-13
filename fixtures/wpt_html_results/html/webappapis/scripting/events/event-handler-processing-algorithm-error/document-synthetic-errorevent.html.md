# html/webappapis/scripting/events/event-handler-processing-algorithm-error/document-synthetic-errorevent.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/scripting/events/event-handler-processing-algorithm-error/document-synthetic-errorevent.html",
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
  document.onerror = t.step_func((...args) => {
    assert_equals(args.length, 1);
    return true;
  });

  const eventWatcher = new EventWatcher(t, document, "error");
  const promise = eventWatcher.wait_for("error").then(e => {
    assert_equals(e.defaultPrevented, false);
    assert_equals(e.message, "");
    assert_equals(e.filename, "");
    assert_equals(e.lineno, 0);
    assert_equals(e.colno, 0);
    assert_equals(e.error, undefined);
  });

  document.dispatchEvent(new ErrorEvent("error", { cancelable: true }));

  return promise;
}, "error event is normal (return true does not cancel; one arg) on Document, with a synthetic ErrorEvent");

test(() => {
  const e = new ErrorEvent("error");
  assert_equals(e.message, "");
  assert_equals(e.filename, "");
  assert_equals(e.lineno, 0);
  assert_equals(e.colno, 0);
  assert_equals(e.error, undefined);
}, "Initial values of ErrorEvent members")

test(() => {
  const e = new ErrorEvent("error", {error : null});
  assert_equals(e.error, null);
}, "error member can be set to null")

test(() => {
  const e = new ErrorEvent("error", {error : undefined});
  assert_equals(e.error, undefined);
}, "error member can be set to undefined")

test(() => {
  const e = new ErrorEvent("error", {error : "foo"});
  assert_equals(e.error, "foo");
}, "error member can be set to arbitrary")

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
  "source_name": "html/webappapis/scripting/events/event-handler-processing-algorithm-error/document-synthetic-errorevent.html"
}
```
