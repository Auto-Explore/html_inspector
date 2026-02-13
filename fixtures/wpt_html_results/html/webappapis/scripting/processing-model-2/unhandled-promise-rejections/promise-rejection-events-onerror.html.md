# html/webappapis/scripting/processing-model-2/unhandled-promise-rejections/promise-rejection-events-onerror.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/scripting/processing-model-2/unhandled-promise-rejections/promise-rejection-events-onerror.html",
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
<link rel="help" href="https://html.spec.whatwg.org/#runtime-script-errors">
<link rel="help" href="https://html.spec.whatwg.org/#unhandled-promise-rejections">
<script>
'use strict';
setup({
  allow_uncaught_exception: true
});
async_test(function(t) {
  var e = new Error('e');
  var e2 = new Error('e2');

  window.onerror = function (msg, url, line, col, error) {
    t.step(function() {
      assert_true(msg.includes('e2'));
      assert_equals(error, e2);
    });
    t.done();
  };

  window.onrejectionhandled = function() {
    // This should cause onerror
    throw e2;
  };

  var p = Promise.reject(e);
  queueTask(function() {
    queueTask(t.step_func(function() {
      // This will cause onrejectionhandled
      p.catch(function() {});
    }));
  });
}, 'Throwing inside an unhandledrejection handler invokes the error handler.');

// This function queues a task in "DOM manipulation task source"
function queueTask(f) {
  var d = document.createElement("details");
  d.ontoggle = function() {
    f();
  };

  d.setAttribute("open", "");
}
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
  "source_name": "html/webappapis/scripting/processing-model-2/unhandled-promise-rejections/promise-rejection-events-onerror.html"
}
```
