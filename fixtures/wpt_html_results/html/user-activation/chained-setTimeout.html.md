# html/user-activation/chained-setTimeout.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/user-activation/chained-setTimeout.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
  <script src="/resources/testharness.js"></script>
  <script src="/resources/testharnessreport.js"></script>
  <script src="/resources/testdriver.js"></script>
  <script src="/resources/testdriver-vendor.js"></script>
  <script>
    let chained_timeout_test = async_test("Chained setTimeout test");

    const max_call_depth = 3;
    const delay_ms = 10;

    function testInitialStates(depth) {
        assert_true(1 <= depth && depth <= max_call_depth);

        chained_timeout_test.step_timeout(() => {
            let test_name = "Call-depth=" + depth + ": initial activation states are false";
            test(() => {
                assert_false(navigator.userActivation.isActive);
                assert_false(navigator.userActivation.hasBeenActive);
            }, test_name);

            if (depth < max_call_depth)
                testInitialStates(depth+1);
            else
                test_driver.click(document.body);
        }, delay_ms);
    }

    function testFinalStates(depth) {
        assert_true(1 <= depth && depth <= max_call_depth);

        chained_timeout_test.step_timeout(() => {
            let test_name = "Call-depth=" + depth + ": after-click activation states are true";
            test(() => {
                assert_true(navigator.userActivation.isActive);
                assert_true(navigator.userActivation.hasBeenActive);
            }, test_name);

            if (depth < max_call_depth)
                testFinalStates(depth+1);
            else
                chained_timeout_test.done();
        }, delay_ms)
    }

    function run() {
        window.addEventListener("click", event => {
            testFinalStates(1);
        });

        testInitialStates(1);
    }
  </script>
</head>
<body onload="run()">
  <h1>User activation state in chained setTimeout calls</h1>
  <p>Tests that user activation state is visible in arbitrary call depth of setTimeout.</p>
  <ol id="instructions">
    <li>Click anywhere in the document.
  </ol></body>
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
  "source_name": "html/user-activation/chained-setTimeout.html"
}
```
