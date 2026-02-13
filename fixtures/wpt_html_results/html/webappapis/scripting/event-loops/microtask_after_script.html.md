# html/webappapis/scripting/event-loops/microtask_after_script.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/scripting/event-loops/microtask_after_script.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<head>
<link rel=author title="Aleks Totic" href="mailto:atotic@chromium.org">
<link rel=help href="https://html.spec.whatwg.org/#clean-up-after-running-script">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="resources/common.js"></script>
</head>
<body style="height:2000px;">
<script>
/*
promise 1, promise 2 execute immediately after script tag
promise 1 child executes immediately after promise 2.

Relevant specs:

https://html.spec.whatwg.org/#clean-up-after-running-script
If the JavaScript execution context stack is now empty, perform a microtask checkpoint.

https://html.spec.whatwg.org/#perform-a-microtask-checkpoint
"perform a microtask checkpoint" runs in a loop until all microtasks have been delivered.
*/

var test = async_test("Microtask immediately after script");

var events = [];

Promise.resolve()
.then(function() {
    events.push("promise 1");
    return Promise.resolve();
})
.then(function() {
    test.step(function() {
        events.push("promise 1 child");
        assert_array_equals(events, ["promise 1", "promise 2", "promise 1 child"]);
        test.done();
    });
});
Promise.resolve()
.then(function() {
    events.push("promise 2");
});

// Set up events that must be executed after Promise.
window.setTimeout(function() {
    events.push('timeout');
}, 0);
window.addEventListener('scroll', function() {
    events.push('scroll');
});
window.scrollBy(0,10);

</script>
</body>
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
  "source_name": "html/webappapis/scripting/event-loops/microtask_after_script.html"
}
```
