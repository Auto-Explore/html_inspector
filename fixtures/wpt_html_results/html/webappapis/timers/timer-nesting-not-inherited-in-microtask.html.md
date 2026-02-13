# html/webappapis/timers/timer-nesting-not-inherited-in-microtask.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/timers/timer-nesting-not-inherited-in-microtask.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<link rel="help" href="https://html.spec.whatwg.org/multipage/webappapis.html#perform-a-microtask-checkpoint">
<link rel="help" href="https://html.spec.whatwg.org/multipage/timers-and-user-prompts.html#timer-initialisation-steps">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
// queue a microtask after the timer has reached the spec-defined maximum nesting level. Then we ensure
// the new timer did not inherit the nesting level from the outer timer task by checking that the sub-4ms
// timeout was not clamped to 4ms.

test(() => {
    assert_equals(typeof performance.now, "function");
}, "Test requires performance.now() to measure approximate timing of callbacks.");


let t = async_test("Test that a timer scheduled during a microtask checkpoint does not inherit the timer nesting level of the task that just ran.");

var rescheduleTimeoutCalledCount = 0;
function rescheduleTimeout()
{
    if (++rescheduleTimeoutCalledCount > 15)
        return t.done();
    else if (rescheduleTimeoutCalledCount > 5) {
        queueMicrotask(() => {
            const checkNotNestedScheduledAt = performance.now();
            setTimeout(() => {
                const approximateDelay = performance.now() - checkNotNestedScheduledAt;
                t.step(() => assert_less_than(approximateDelay, 4, "Timer should not be clamped to 4ms"));
            }, 1);
        });
    }
    setTimeout(rescheduleTimeout, 8);
}

window.addEventListener("load", () => setTimeout(rescheduleTimeout, 8));
</script>
</html>
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
  "source_name": "html/webappapis/timers/timer-nesting-not-inherited-in-microtask.html"
}
```
