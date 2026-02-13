# html/browsers/browsing-the-web/history-traversal/popstate_event.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/history-traversal/popstate_event.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Queue a task to fire popstate event</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<script>
t = async_test();
window.onload = t.step_func(function () {
  var states = [];

  var timer = null;

  history.pushState("a", "State a", "/a");
  history.pushState("b", "State b", "/b");

  history.back();
  window.onpopstate = t.step_func(function (e) {
    assert_true(e.isTrusted, "isTrusted");
    assert_equals(e.target, window, "target");
    assert_equals(e.type, "popstate", "type");
    assert_true(e instanceof PopStateEvent, "is PopStateEvent");
    assert_false(e.bubbles, "bubbles");
    assert_false(e.cancelable, "cancelable");
    assert_not_equals(e.hasUAVisualTransition, undefined);

    states.push(e.state);

    if (states.length === 2) {
      check_result();
    } else if (timer === null) {
      timer = setTimeout(function() {check_result()}, 500);
    }
  })

  check_result = t.step_func(function() {
    clearTimeout(timer);
    try {
      assert_array_equals(states, ["a", null]);
      t.done();
    } finally {
      location.hash = "";
    }
  });

  setTimeout(function() {history.back()}, 0);

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
  "source_name": "html/browsers/browsing-the-web/history-traversal/popstate_event.html"
}
```
