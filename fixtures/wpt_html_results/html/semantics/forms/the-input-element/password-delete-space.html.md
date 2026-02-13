# html/semantics/forms/the-input-element/password-delete-space.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-input-element/password-delete-space.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Backspace with trailing white space in password field</title>
<link rel="help" href="https://bugs.chromium.org/p/chromium/issues/detail?id=1400844">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#password-state-%28type=password%29">
<link rel="author" href="mailto:xiaochengh@chromium.org">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>

<input id="target" type="password" value="   ">

<script>
promise_test(async () => {
  target.focus();
  target.selectionStart = 2;
  await test_driver.send_keys(target, '\uE003');
  assert_equals(target.value, "  ");
}, "Backspace with trailing white space in password field");
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
  "source_name": "html/semantics/forms/the-input-element/password-delete-space.html"
}
```
