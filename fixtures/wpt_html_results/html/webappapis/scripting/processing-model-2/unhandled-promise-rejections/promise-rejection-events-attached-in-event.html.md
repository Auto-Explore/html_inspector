# html/webappapis/scripting/processing-model-2/unhandled-promise-rejections/promise-rejection-events-attached-in-event.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/scripting/processing-model-2/unhandled-promise-rejections/promise-rejection-events-attached-in-event.html",
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
<link rel="help" href="https://html.spec.whatwg.org/#unhandled-promise-rejections">
<script>
'use strict';
setup({
  allow_uncaught_exception: true
});
async_test(function(t) {
  var e = new Error('e');
  var p = Promise.reject(e);

  window.onunhandledrejection = function(evt) {
    t.step(function() {
      assert_equals(evt.promise, p);
      assert_equals(evt.reason, e);
    });
    var unreached = t.unreached_func('promise should not be fulfilled');
    p.then(unreached, function(reason) {
      t.step(function() {
        assert_equals(reason, e);
      });
      t.step_timeout(function() { t.done(); }, 10);
    });
  };

  window.onrejectionhandled = t.unreached_func('rejectionhandled event should not be invoked');
}, 'Attaching a handler in unhandledrejection should not trigger rejectionhandled.');
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
  "source_name": "html/webappapis/scripting/processing-model-2/unhandled-promise-rejections/promise-rejection-events-attached-in-event.html"
}
```
