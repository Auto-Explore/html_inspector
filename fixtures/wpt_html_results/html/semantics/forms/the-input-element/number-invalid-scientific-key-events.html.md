# html/semantics/forms/the-input-element/number-invalid-scientific-key-events.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-input-element/number-invalid-scientific-key-events.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Input Type Number: Reject Invalid Scientific Notation Key Events</title>+
<link rel="help" href="https://crbug.com/428657806">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="/resources/testdriver-actions.js"></script>

<input type="number">

<script>
const input = document.querySelector('input');
const inputNumber = "1e+1";

async function perform_key_action(caret_moves_left, key_to_insert) {
  // Reset input.
  input.value = "";
  input.blur();
  assert_true(input.validity.valid, "Input should start valid");

  // Type initial value.
  input.focus();
  await test_driver.send_keys(input, inputNumber);

  // Caret at end, move to the left.
  for (let i = 0; i < caret_moves_left; i++) {
    await test_driver.send_keys(input, "\uE012"); // ArrowLeft
  }

  // Insert key.
  await test_driver.send_keys(input, key_to_insert);

  assert_true(input.validity.valid, "Input should remain valid after insertion attempt");
}

promise_test(async () => {
  // Move to position after 'e'.
  await perform_key_action(2, "-");

  assert_equals(input.valueAsNumber, 10, "Should reject '-' after 'e' in '1e+1'");
}, "Reject multiple signs after 'e' (-)");

promise_test(async () => {
  // Move to position after 'e'.
  await perform_key_action(2, "+");

  assert_equals(input.valueAsNumber, 10, "Should reject '+' after 'e' in '1e+1'");
}, "Reject multiple signs after 'e' (+)");


promise_test(async () => {
  // Move to start.
  await perform_key_action(4, "+");

  assert_equals(input.valueAsNumber, 10, "Leading '+' with existing 'e' should result in a valid number");
}, "Leading '+' with existing 'e'");

promise_test(async () => {
  // Move to start.
  await perform_key_action(4, "-");

  assert_equals(input.valueAsNumber, -10, "Should accept leading '-'");
}, "Insert - at start");

</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.link.in_body.disallowed_rel",
      "message": "A “link” element must not appear as a descendant of a “body” element unless the “link” element has an “itemprop” attribute or has a “rel” attribute whose value contains “dns-prefetch”, “modulepreload”, “pingback”, “preconnect”, “prefetch”, “preload”, “prerender”, or “stylesheet”.",
      "severity": "Warning",
      "span": {
        "byte_end": 172,
        "byte_start": 120,
        "col": 1,
        "line": 4
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/forms/the-input-element/number-invalid-scientific-key-events.html"
}
```
