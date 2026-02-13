# html/user-activation/activation-trigger-keyboard-escape.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/user-activation/activation-trigger-keyboard-escape.html",
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
  <h1>Test for keyboard activation trigger for ESCAPE key</h1>
  <p>Tests missing user activation from a ESCAPE keyboard event.</p>
  <input type="text" autofocus />
  <ol id="instructions">
    <li>Press ESCAPE key.
  </ol>
  <script>
  function runTests() {
    promise_test(async () => {
        const ESCAPE_KEY = '\uE00C';

        let keydown_event = getEvent('keydown');
        let keyup_event = getEvent('keyup');

        await test_driver.send_keys(document.body, ESCAPE_KEY);

        await keydown_event;
        let consumed = await consumeTransientActivation();
        assert_false(consumed,
                    "ESCAPE keydown event should not result in activation");

        await keyup_event;
        consumed = await consumeTransientActivation();
        assert_false(consumed,
                     "ESCAPE keyup should have no activation after keydown consumption");
    }, "Activation through ESCAPE keyboard event");
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
  "source_name": "html/user-activation/activation-trigger-keyboard-escape.html"
}
```
