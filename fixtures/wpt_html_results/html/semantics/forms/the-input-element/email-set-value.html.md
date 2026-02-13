# html/semantics/forms/the-input-element/email-set-value.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-input-element/email-set-value.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Input Email setValue</title>
<link rel="author" href="mailto:atotic@chromium.org">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#e-mail-state-(type=email)">
<link rel="help" href="https://crbug.com/423785">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="/resources/testdriver-vendor.js"></script>

<input type="email">

<script>
promise_test(async () => {
  let input = document.querySelector("input");
  let unsanitized = ' foo@bar   ';
  let sanitized = unsanitized.trim();
  await test_driver.send_keys(input, unsanitized);
  assert_true(input.validity.valid, "unsanitized input is valid");
  input.select();
  assert_equals(input.value, sanitized, "value is sanitized");
  assert_equals(window.getSelection().toString(), unsanitized,
    "visible value is unsanitized");
  input.value = sanitized;
  input.select();
  assert_equals(window.getSelection().toString(), sanitized,
    "visible value is sanitized after setValue(sanitized)");
},
"setValue(sanitizedValue) is reflected in visible text field content");
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
  "source_name": "html/semantics/forms/the-input-element/email-set-value.html"
}
```
