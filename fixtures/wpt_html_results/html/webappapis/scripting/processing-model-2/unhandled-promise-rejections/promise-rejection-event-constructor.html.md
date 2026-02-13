# html/webappapis/scripting/processing-model-2/unhandled-promise-rejections/promise-rejection-event-constructor.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/scripting/processing-model-2/unhandled-promise-rejections/promise-rejection-event-constructor.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<link rel="help" href="https://html.spec.whatwg.org/#the-promiserejectionevent-interface">
<script>
'use strict';

test(function() {
  var p = new Promise(function(resolve, reject) {});

  assert_throws_js(TypeError,
                   function() {
                     PromiseRejectionEvent('', { promise: p });
                   },
                   "Calling PromiseRejectionEvent constructor without 'new' must throw");

  // No custom options are passed (besides required promise).
  assert_equals(new PromiseRejectionEvent('eventType', { promise: p }).bubbles, false);
  assert_equals(new PromiseRejectionEvent('eventType', { promise: p }).cancelable, false);
  assert_equals(new PromiseRejectionEvent('eventType', { promise: p }).promise, p);
  assert_equals(new PromiseRejectionEvent('eventType', { promise: p }).reason, undefined);

  // No promise is passed.
  assert_throws_js(TypeError,
                   function() {
                     new PromiseRejectionEvent('eventType', { bubbles: false });
                   },
                   'Cannot construct PromiseRejectionEventInit without promise');

  // bubbles is passed.
  assert_equals(new PromiseRejectionEvent('eventType', { bubbles: false, promise: p }).bubbles, false);
  assert_equals(new PromiseRejectionEvent('eventType', { bubbles: true, promise: p }).bubbles, true);

  // cancelable is passed.
  assert_equals(new PromiseRejectionEvent('eventType', { cancelable: false, promise: p }).cancelable, false);
  assert_equals(new PromiseRejectionEvent('eventType', { cancelable: true, promise: p }).cancelable, true);

  // reason is passed.
  var r = new Error();
  assert_equals(new PromiseRejectionEvent('eventType', { promise: p, reason: r }).reason, r);
  assert_equals(new PromiseRejectionEvent('eventType', { promise: p, reason: null }).reason, null);

  // All initializers are passed.
  assert_equals(new PromiseRejectionEvent('eventType', { bubbles: true, cancelable: true, promise: p, reason: r }).bubbles, true);
  assert_equals(new PromiseRejectionEvent('eventType', { bubbles: true, cancelable: true, promise: p, reason: r }).cancelable, true);
  assert_equals(new PromiseRejectionEvent('eventType', { bubbles: true, cancelable: true, promise: p, reason: r }).promise, p);
  assert_equals(new PromiseRejectionEvent('eventType', { bubbles: true, cancelable: true, promise: p, reason: r }).reason, r);
}, "This tests the constructor for the PromiseRejectionEvent DOM class.");
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.head.title.missing",
      "message": "Element “head” is missing a required instance of child element “title”.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/webappapis/scripting/processing-model-2/unhandled-promise-rejections/promise-rejection-event-constructor.html"
}
```
