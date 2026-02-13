# html/user-activation/activation-trigger-mouse-right.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/user-activation/activation-trigger-mouse-right.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
  <link rel="help" href="https://html.spec.whatwg.org/multipage/interaction.html#activation-triggering-input-event">
  <script src="/resources/testharness.js"></script>
  <script src="/resources/testharnessreport.js"></script>
  <script src="/resources/testdriver.js"></script>
  <script src="/resources/testdriver-actions.js"></script>
  <script src="/resources/testdriver-vendor.js"></script>
  <script src="resources/utils.js"></script>
</head>
<body onload="runTests()">
  <h1>Test for right-click activation trigger</h1>
  <p>Tests user activation from a mouse right-click.</p>
  <ol id="instructions">
    <li>Right-click anywhere in the document.
  </ol>
  <script>
  function runTests() {
    promise_test(async () => {
        var actions = new test_driver.Actions();
        actions.pointerMove(0, 0, {origin: document.body})
            .pointerDown({button: actions.ButtonType.RIGHT})
            .pointerUp({button: actions.ButtonType.RIGHT})
            .send();

        // In most non-Windows platforms the right-click context-menu appears on mousedown, so
        // mouseup and auxclick events are not received by the page if the menu is modal.  We
        // are suppressing the context-menu to guarantee receiving those later events.
        document.body.addEventListener("contextmenu", e => e.preventDefault());

        let mousedown_event = getEvent('mousedown');
        let mouseup_event = getEvent('mouseup');
        let auxclick_event = getEvent('auxclick');
        let contextmenu_event = getEvent('contextmenu');

        await mousedown_event;
        let consumed = await consumeTransientActivation();
        assert_true(consumed,
                    "mousedown event should result in activation");

        await mouseup_event;
        consumed = await consumeTransientActivation();
        assert_false(consumed,
                     "mouseup should have no activation after mousedown consumption");

        await auxclick_event;
        consumed = await consumeTransientActivation();
        assert_false(consumed,
                     "auxclick should have no activation after mousedown consumption");

        await contextmenu_event;
        consumed = await consumeTransientActivation();
        assert_false(consumed,
                     "contextmenu should have no activation after mousedown consumption");
    }, "Activation through right-click mouse event");
  }
  </script>
</body>
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
  "source_name": "html/user-activation/activation-trigger-mouse-right.html"
}
```
