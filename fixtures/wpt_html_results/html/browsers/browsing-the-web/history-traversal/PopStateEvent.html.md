# html/browsers/browsing-the-web/history-traversal/PopStateEvent.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/history-traversal/PopStateEvent.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<title>Synthetic popstate events</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<script>
test(function() {
  assert_throws_js(
    TypeError,
    () => PopStateEvent(''),
    "Calling PopStateEvent constructor without 'new' must throw"
  );
}, "PopStateEvent constructor called as normal function");

test(function () {
  assert_false('initPopStateEvent' in PopStateEvent.prototype,
               'There should be no PopStateEvent#initPopStateEvent');
}, 'initPopStateEvent');

test(function () {
  var popStateEvent = new PopStateEvent("popstate");
  assert_equals(popStateEvent.state, null, "the PopStateEvent.state");
}, "Initial value of PopStateEvent.state must be null");

test(function () {
  var popStateEvent = new PopStateEvent("popstate");
  assert_false(popStateEvent.hasUAVisualTransition, "the PopStateEvent.hasUAVisualTransition");
}, "Initial value of PopStateEvent.hasUAVisualTransition must be false");

test(function () {
  var state = history.state;
  var data;
  var hasUAVisualTransition = false;
  window.addEventListener('popstate', function (e) {
    data = e.state;
    hasUAVisualTransition = e.hasUAVisualTransition;
  });
  window.dispatchEvent(new PopStateEvent('popstate', {
    'state': {testdata:true},
    'hasUAVisualTransition': true
  }));
  assert_true(data.testdata,'state data was corrupted');
  assert_equals(history.state, state, "history.state was NOT set by dispatching the event");
  assert_true(hasUAVisualTransition, 'hasUAVisualTransition not set correctly');
}, 'Dispatching a synthetic PopStateEvent');
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
  "source_name": "html/browsers/browsing-the-web/history-traversal/PopStateEvent.html"
}
```
