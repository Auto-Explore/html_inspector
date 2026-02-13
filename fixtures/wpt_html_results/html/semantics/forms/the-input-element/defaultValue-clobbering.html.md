# html/semantics/forms/the-input-element/defaultValue-clobbering.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-input-element/defaultValue-clobbering.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<link rel="author" title="Joey Arhar" href="mailto:jarhar@chromium.org">
<meta name="assert" content="Assigning to defaultValue does not modify text a user has already typed in.">

<!-- This behavior is not explicitly specified. -->

<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>

<div>
  email with leading whitespace: <input id=emailinput type=email>
</div>
<div>
  number with trailing incomplete exponent: <input id=numberinput type=number>
</div>

<script>
promise_test(async () => {
  await test_driver.send_keys(emailinput, '  user');
  assert_false(emailinput.validity.valid, '"  user" should not be a valid value for type=email.');

  emailinput.defaultValue = emailinput.value;
  assert_false(emailinput.validity.valid, 'Assigning to defaultValue should not affect input.validity.');
}, 'Visible value and validity should not be affected when assigning to the defaultValue property for type=email.');

promise_test(async () => {
  await test_driver.send_keys(numberinput, '123e');
  assert_false(numberinput.validity.valid, '"123e" should not be a valid value for type=number.');

  numberinput.defaultValue = numberinput.value;
  assert_false(numberinput.validity.valid, 'Assigning to defaultValue should not affect input.validity.');
}, 'Visible value and validity should not be affected when assigning to the defaultValue property for type=number.');
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
  "source_name": "html/semantics/forms/the-input-element/defaultValue-clobbering.html"
}
```
