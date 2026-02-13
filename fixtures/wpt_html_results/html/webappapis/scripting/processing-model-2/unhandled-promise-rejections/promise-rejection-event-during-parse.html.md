# html/webappapis/scripting/processing-model-2/unhandled-promise-rejections/promise-rejection-event-during-parse.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/scripting/processing-model-2/unhandled-promise-rejections/promise-rejection-event-during-parse.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Promise rejection during initial parsing of document</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<link rel="help" href="https://html.spec.whatwg.org/#unhandled-promise-rejections">
<body>
<p>The script in this test is executed immediately while parsing is ongoing, and
<a
href="https://html.spec.whatwg.org/multipage/webappapis.html#clean-up-after-running-script">cleaning
up after running script</a> involves queueing a task on the DOM manipulation
task source to fire the <code>unhandledrejection</code> event. Parsing then
completes, immediately transitioning the document's readiness state to
"interactive," and queuing another task on the DOM manipulation task source to
transition the state to "complete."
</p>
<script>
'use strict';
setup({ allow_uncaught_exception: true });

async_test(function(t) {
  const events = [];
  document.addEventListener('readystatechange', t.step_func(function() {
    events.push('readystatechange:' + document.readyState);
  }));
  addEventListener('unhandledrejection', t.step_func(function() {
    events.push('unhandledrejection');
  }));

  Promise.reject(new Error('this error is intentional'));

  addEventListener('load', t.step_func(function() {
    assert_array_equals(
      events,
      [
        'readystatechange:interactive',
        'unhandledrejection',
        'readystatechange:complete'
      ]
    );
    t.done();
  }));
});
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
  "source_name": "html/webappapis/scripting/processing-model-2/unhandled-promise-rejections/promise-rejection-event-during-parse.html"
}
```
