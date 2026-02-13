# html/semantics/forms/the-input-element/number-constraint-validation.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-input-element/number-constraint-validation.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Form input type=number constraint validation</title>
<link rel="author" title="Adam Vandolder" href="mailto:avandolder@mozilla.com">
<link rel=help href="https://html.spec.whatwg.org/multipage/#number-state-(type=number)">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<div id="log"></div>
<input type="number">
<script>
  const input = document.querySelector("input");
  const invalidInputNumber = "1.e";
  const invalidSetNumber = "1.";

  promise_test(async () => {
    await test_driver.click(input);
    await test_driver.send_keys(input, invalidInputNumber);
    assert_equals(input.value.length, 0);
    assert_true(input.validity.badInput);
  }, "Unparsable number user input triggers sanitization and causes badInput to be set.");

  promise_test(async () => {
    input.value = invalidInputNumber;
    assert_equals(input.value.length, 0);
    assert_false(input.validity.badInput);
  }, "Setting .value to an unparsable number triggers sanitization but doesn't set badInput.");

  promise_test(async () => {
    await test_driver.click(input);
    await test_driver.send_keys(input, invalidSetNumber);
    assert_equals(input.value, "1");
    assert_false(input.validity.badInput);
  }, "Users inputting a parsable but invalid floating point number doesn't trigger sanitization and doesn't set badInput.");

  promise_test(async () => {
    input.value = invalidSetNumber;
    assert_equals(input.value.length, 0);
    assert_false(input.validity.badInput);
  }, "Setting .value to a parsable but invalid floating point number triggers sanitization but doesn't set badInput.");
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
  "source_name": "html/semantics/forms/the-input-element/number-constraint-validation.html"
}
```
