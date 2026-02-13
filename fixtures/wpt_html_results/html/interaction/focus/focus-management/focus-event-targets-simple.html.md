# html/interaction/focus/focus-management/focus-event-targets-simple.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/interaction/focus/focus-management/focus-event-targets-simple.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
  <meta charset="utf-8">
  <title>Focus events fire at correct targets in correct order in simple case</title>
  <link rel="author" title="Chris Rebert" href="http://chrisrebert.com">
  <link rel="help" href="https://html.spec.whatwg.org/#focus-update-steps">
  <link rel="help" href="https://html.spec.whatwg.org/#focus-chain">
  <meta name="flags" content="dom">
  <script src="/resources/testharness.js"></script>
  <script src="/resources/testharnessreport.js"></script>
</head>
<body>
  <input type="text" id="a">
  <script>
// Record all the focus event targets in an array.
// Modulo special cases in the "focus update steps" algorithm,
// this should be the same as the new focus chain, except in reverse order.
var newFocusChainReversedNotQuite = [];
var pushTarget = function (e) {
  newFocusChainReversedNotQuite.push(e.target);
};
// Window is the root node for event dispatch per https://html.spec.whatwg.org/multipage/webappapis.html#events-and-the-window-object
window.addEventListener('focus', pushTarget, true);// Use event capturing since focus event doesn't bubble
var input = document.getElementById('a');
input.focus();
window.removeEventListener('focus', pushTarget, true);
test(function() {
  assert_array_equals(newFocusChainReversedNotQuite, [input], "Exactly 1 focus event should fire and its target should be the input");
}, "Focus events fire at correct targets in correct order in simple case");
  </script>
</body>
</html>
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
  "source_name": "html/interaction/focus/focus-management/focus-event-targets-simple.html"
}
```
