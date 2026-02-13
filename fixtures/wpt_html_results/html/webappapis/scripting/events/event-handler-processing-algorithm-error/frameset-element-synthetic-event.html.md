# html/webappapis/scripting/events/event-handler-processing-algorithm-error/frameset-element-synthetic-event.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/scripting/events/event-handler-processing-algorithm-error/frameset-element-synthetic-event.html",
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

<iframe name="framesetWindow" src="resources/frameset-frame.html"></iframe>
<div id="log"></div>

<script>
"use strict";
setup({ allow_uncaught_exception: true });

window.onload = () => {

const frameset = framesetWindow.document.querySelector("frameset");

promise_test(t => {
  frameset.onerror = t.step_func((...args) => {
    assert_equals(args.length, 1);
    return true;
  });

  const eventWatcher = new EventWatcher(t, framesetWindow, "error");
  const promise = eventWatcher.wait_for("error").then(e => {
    assert_equals(e.defaultPrevented, false);
  });

  frameset.dispatchEvent(new Event("error", { bubbles: true, cancelable: true }));

  return promise;
}, "error event is normal (return true does not cancel; one arg) on Window, with a synthetic Event");

};
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
  "source_name": "html/webappapis/scripting/events/event-handler-processing-algorithm-error/frameset-element-synthetic-event.html"
}
```
