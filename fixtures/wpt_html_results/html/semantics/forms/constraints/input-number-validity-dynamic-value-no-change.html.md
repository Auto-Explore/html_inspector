# html/semantics/forms/constraints/input-number-validity-dynamic-value-no-change.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/constraints/input-number-validity-dynamic-value-no-change.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<title>Number input step dynamic value attribute change</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<link rel="help" href="https://bugzilla.mozilla.org/show_bug.cgi?id=1621273">
<input type="number" value="9999" step="1">
<script>
test(function() {
  let input = document.querySelector("input");
  input.value = "113.90";
  assert_true(input.matches(":invalid"), "Input should be invalid because step base is @value");
  assert_false(input.validity.valid, "Input should be invalid because step base is @value");
  input.setAttribute("value", "113.90");
  assert_true(input.matches(":valid"), "Input should be valid because step base is @value");
  assert_true(input.validity.valid, "Input should be valid because step base is @value");
}, "number input number validation is updated correctly after value attribute change which doesn't change input value");
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
  "source_name": "html/semantics/forms/constraints/input-number-validity-dynamic-value-no-change.html"
}
```
