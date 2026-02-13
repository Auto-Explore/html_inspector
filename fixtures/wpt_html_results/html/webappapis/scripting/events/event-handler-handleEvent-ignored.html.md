# html/webappapis/scripting/events/event-handler-handleEvent-ignored.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/scripting/events/event-handler-handleEvent-ignored.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>"handleEvent" property of EventHandler should be ignored</title>
<link rel="help" href="https://html.spec.whatwg.org/#eventhandler">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<script>
"use strict";

test(t => {
  const handler = Object.create(null, {
    handleEvent: {
      get: t.unreached_func('"handleEvent" property should not be looked up'),
    },
  });

  const el = document.createElement("div");
  el.onmouseenter = handler;
  el.dispatchEvent(new MouseEvent("mouseenter"));
}, 'plain object "mouseenter" handler');

async_test(t => {
  const handler = Object.create(Function.prototype, {
    handleEvent: {
      get: t.unreached_func('"handleEvent" property should not be looked up'),
    },
  });
  assert_true(handler instanceof Function);

  window.onmessage = handler;
  window.postMessage({}, "*");

  step_timeout(() => {
    t.done();
  }, 50);
}, 'non-callable "message" handler that is instance of Function');
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
  "source_name": "html/webappapis/scripting/events/event-handler-handleEvent-ignored.html"
}
```
