# html/user-activation/user-activation-interface.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/user-activation/user-activation-interface.html",
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
  <script src="resources/utils.js"></script>
</head>
<body onload="runTests()">
  <h1>Basic test for navigator.userActivation interface</h1>
  <p>Tests that navigator.userActivation shows user activation states.</p>
  <ol id="instructions">
    <li>Click anywhere in the document.
  </ol>
  <script>
  function runTests() {
    promise_test(async () => {
        assert_true(!!navigator.userActivation, "This test requires navigator.userActivation API");

        assert_false(navigator.userActivation.hasBeenActive, "No sticky activation before click");
        assert_false(navigator.userActivation.isActive, "No transient activation before click");

        await test_driver.click(document.body);

        assert_true(navigator.userActivation.hasBeenActive, "Has sticky activation after click");
        assert_true(navigator.userActivation.isActive, "Has transient activation after click");
    }, "navigator.userActivation shows correct states before/after a click");
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
  "source_name": "html/user-activation/user-activation-interface.html"
}
```
