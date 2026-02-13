# html/semantics/popovers/light-dismiss-event-ordering.tentative.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/popovers/light-dismiss-event-ordering.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=help href="https://chromium-review.googlesource.com/c/chromium/src/+/4023021">
<link rel=help href="https://github.com/whatwg/html/pull/8221#discussion_r1041135388">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="resources/popover-utils.js"></script>

<button id=target>target</button>
<div id=popover popover=auto>popover</div>

<script>
for (const capture of [true, false]) {
  for (const eventName of ['pointerdown', 'pointerup', 'mousedown', 'mouseup', 'click']) {
    promise_test(async t => {
      t.add_cleanup(() => {
        try {
          popover.hidePopover();
        } catch {}
      });

      popover.showPopover();
      document.addEventListener(eventName, event => {
        event.preventDefault();
      }, {capture, once: true});
      // Click away from the popover to activate light dismiss.
      await clickOn(target);
      assert_equals(document.querySelectorAll(':popover-open').length, 0,
        'The popover should be closed via light dismiss even when preventDefault is called.');

      popover.showPopover();
      document.addEventListener(eventName, event => {
        event.stopPropagation();
      }, {capture, once: true});
      // Click away from the popover to activate light dismiss.
      await clickOn(target);
      assert_equals(document.querySelectorAll(':popover-open').length, 0,
        'The popover should be closed via light dismiss even when stopPropagation is called.');

    }, `Tests the interactions between popover light dismiss and pointer/mouse events. eventName: ${eventName}, capture: ${capture}`);
  }
}

promise_test(async t => {
  t.add_cleanup(() => {
    try {
      popover.hidePopover();
    } catch {}
  });
  popover.showPopover();

  const expectedEvents = [
    'pointerdown',
    'mousedown',
    'pointerup',
    'mouseup',
    'beforetoggle newState: closed',
    'click'
  ];
  const events = [];

  for (const eventName of ['pointerdown', 'pointerup', 'mousedown', 'mouseup', 'click']) {
    document.addEventListener(eventName, () => events.push(eventName));
  }
  popover.addEventListener('beforetoggle', event => {
    events.push('beforetoggle newState: ' + event.newState);
  });

  // Click away from the popover to activate light dismiss.
  await clickOn(target);

  assert_array_equals(events, expectedEvents,
    'pointer and popover events should be fired in the correct order.');

  assert_equals(document.querySelectorAll(':popover-open').length, 0,
    'The popover should be closed via light dismiss.');

}, 'Tests the order of pointer/mouse events during popover light dismiss.');
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
  "source_name": "html/semantics/popovers/light-dismiss-event-ordering.tentative.html"
}
```
