# html/semantics/forms/the-input-element/date-readonly-keyboard.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-input-element/date-readonly-keyboard.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<link rel="author" href="mailto:masonf@chromium.org">
<link rel="help" href="https://crbug.com/478905605">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="/resources/testdriver-actions.js"></script>

<input type="date" readonly value="1970-01-01">

<script>
promise_test(async t => {
  const input = document.querySelector('input');
  input.focus();

  // Press Space to open picker (if bug exists)
  await test_driver.send_keys(input, " ");

  // Press Down Arrow
  await test_driver.send_keys(input, "\uE015"); // ArrowDown

  // Press Enter
  await test_driver.send_keys(input, "\uE007"); // Enter

  assert_equals(input.value, "1970-01-01", "Readonly input value should not change");
}, "Readonly date input should not change value via keyboard interaction");
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
  "source_name": "html/semantics/forms/the-input-element/date-readonly-keyboard.html"
}
```
