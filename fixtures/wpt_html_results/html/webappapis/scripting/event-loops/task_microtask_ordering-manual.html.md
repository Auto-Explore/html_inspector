# html/webappapis/scripting/event-loops/task_microtask_ordering-manual.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/scripting/event-loops/task_microtask_ordering-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Task and Microtask Ordering </title>
<link rel=author title="Joshua Bell" href="mailto:jsbell@google.com">
<link rel=help href="https://html.spec.whatwg.org/multipage/#event-loops">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="resources/common.js"></script>
<style>
.inner { padding: 46px; width: 0; margin: 0 auto; background: #d4d4d4; }
.outer { padding: 25px; width: 92px; background: #f1f1f1; }
</style>

<p>Click on the inner box:</p>
<div class="outer">
  <div class="inner"></div>
</div>

<script>

// Based on: https://jakearchibald.com/2015/tasks-microtasks-queues-and-schedules/

log_test(function(t, log) {
    // Let's get hold of those elements
    var outer = document.querySelector('.outer');
    var inner = document.querySelector('.inner');

    // Let's listen for attribute changes on the
    // outer element
    new MutationObserver(function() {
        log('mutate');
    }).observe(outer, {
        attributes: true
    });

    // Here's a click listener...
    function onClick() {
        log('click');

        setTimeout(function() {
            log('timeout');
        }, 0);

        Promise.resolve().then(function() {
            log('promise');
        });

        outer.setAttribute('data-random', Math.random());
    }

    // ...which we'll attach to both elements
    inner.addEventListener('click', onClick);
    outer.addEventListener('click', onClick);
}, [
    'click',
    'promise',
    'mutate',
    'click',
    'promise',
    'mutate',
    'timeout',
    'timeout'
], 'Level 1 bossfight (manual click)');

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
  "source_name": "html/webappapis/scripting/event-loops/task_microtask_ordering-manual.html"
}
```
