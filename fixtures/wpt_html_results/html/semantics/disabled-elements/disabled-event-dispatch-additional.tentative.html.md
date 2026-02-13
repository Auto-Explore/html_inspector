# html/semantics/disabled-elements/disabled-event-dispatch-additional.tentative.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/disabled-elements/disabled-event-dispatch-additional.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta name="timeout" content="long">
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=help href="https://github.com/whatwg/html/issues/2368">
<link rel=help href="https://github.com/whatwg/html/issues/5886">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="/resources/testdriver-actions.js"></script>

<!-- This test should be merged with disabled-event-dispatch.tentative.html after interop2023 is over. -->

<div id=targetparent>
  <button disabled>
    hello world
    <span style="border: 1px solid black">child</span>
  </button>
  <my-control disabled>
    hello world
    <span style="border: 1px solid black">child</span>
  </my-control>
</div>

<script>
customElements.define('my-control', class extends HTMLElement {
  static get formAssociated() { return true; }
});

['dblclick', 'auxclick'].forEach(eventName => {
  [true, false].forEach(clickChildElement => {
    for (const target of targetparent.children) {
      promise_test(async () => {
        let parentReceivedEvent = false;
        targetparent.addEventListener(eventName, () => parentReceivedEvent = true);

        let targetReceivedEvent = false;
        target.addEventListener(eventName, () => targetReceivedEvent = true);

        let childReceivedEvent = false;
        let targetchild = target.firstElementChild;
        targetchild.addEventListener(eventName, () => childReceivedEvent = true);

        const elementToClick = clickChildElement ? targetchild : target;
        if (eventName === 'dblclick') {
          await (new test_driver.Actions()
            .pointerMove(1, 1, {origin: elementToClick})
            .pointerDown()
            .pointerUp()
            .pointerDown()
            .pointerUp())
            .send();
        } else if (eventName === 'auxclick') {
          const actions = new test_driver.Actions();
          await actions
            .pointerMove(1, 1, {origin: elementToClick})
            .pointerDown({button: actions.ButtonType.MIDDLE})
            .pointerUp({button: actions.ButtonType.MIDDLE})
            .send();
        }


        const shouldReceiveEvents = eventName.startsWith('pointer') || eventName === 'auxclick';
        assert_equals(parentReceivedEvent, shouldReceiveEvents,
                      `parent element received ${eventName} events`);
        assert_equals(targetReceivedEvent, shouldReceiveEvents,
                      `target element received ${eventName} events`);
        assert_equals(childReceivedEvent, clickChildElement,
                      `child element received ${eventName} events`);
      }, `Testing ${eventName} events when clicking ${clickChildElement ? 'child of ' : ''}disabled ${target.localName}.`);
    }
  });
});
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
  "source_name": "html/semantics/disabled-elements/disabled-event-dispatch-additional.tentative.html"
}
```
