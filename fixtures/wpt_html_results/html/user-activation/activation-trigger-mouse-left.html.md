# html/user-activation/activation-trigger-mouse-left.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/user-activation/activation-trigger-mouse-left.html",
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
  <script src="/resources/testdriver-vendor.js"></script>
  <script src="resources/utils.js"></script>
</head>
<body onload="runTests()">
  <h1>Test for click activation trigger</h1>
  <p>Tests user activation from a mouse click.</p>
  <ol id="instructions">
    <li>Click anywhere in the document.
  </ol>
  <script>
  function runTests() {
    promise_test(async () => {

        let mousedown_event = getEvent('mousedown');
        let mouseup_event = getEvent('mouseup');
        let click_event = getEvent('click');

        await test_driver.click(document.body);

        await mousedown_event;
        let consumed = await consumeTransientActivation();
        assert_true(consumed,
                    "mousedown event should result in activation");

        await mouseup_event;
        consumed = await consumeTransientActivation();
        assert_false(consumed,
                     "mouseup should have no activation after mousedown consumption");

        await click_event;
        consumed = await consumeTransientActivation();
        assert_false(consumed,
                     "click should have no activation after mousedown consumption");
    }, "Activation through left-click mouse event");
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
  "source_name": "html/user-activation/activation-trigger-mouse-left.html"
}
```
