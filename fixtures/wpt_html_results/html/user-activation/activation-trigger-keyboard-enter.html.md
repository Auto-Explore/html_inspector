# html/user-activation/activation-trigger-keyboard-enter.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/user-activation/activation-trigger-keyboard-enter.html",
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
  <h1>Test for keyboard activation trigger for ENTER key</h1>
  <p>Tests user activation from a ENTER keyboard event.</p>
  <input type="text" autofocus />
  <ol id="instructions">
    <li>Press ENTER key.
  </ol>
  <script>
  function runTests() {
    promise_test(async () => {
        const ENTER_KEY = '\uE007';

        let keydown_event = getEvent('keydown');
        let keypress_event = getEvent('keypress');
        let keyup_event = getEvent('keyup');

        await test_driver.send_keys(document.body, ENTER_KEY);

        await keydown_event;
        let consumed = await consumeTransientActivation();
        assert_true(consumed,
                    "ENTER keydown event should result in activation");

        await keypress_event;
        consumed = await consumeTransientActivation();
        assert_false(consumed,
                     "ENTER keypress should have no activation after keydown consumption");

        await keyup_event;
        consumed = await consumeTransientActivation();
        assert_false(consumed,
                     "ENTER keyup should have no activation after keydown consumption");
    }, "Activation through ENTER keyboard event");
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
  "source_name": "html/user-activation/activation-trigger-keyboard-enter.html"
}
```
