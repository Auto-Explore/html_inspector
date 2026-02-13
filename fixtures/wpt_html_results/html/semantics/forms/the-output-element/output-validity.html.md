# html/semantics/forms/the-output-element/output-validity.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-output-element/output-validity.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<title>:valid and :invalid pseudo-class on output element</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<output id='output_test'></output>

<script>

test(() => {
  let output = document.getElementById("output_test");
  assert_false(output.matches(":valid"), "should not match :valid pseudo-class");
  assert_false(output.matches(":invalid"), "should not match :invalid pseudo-class");

  output.setCustomValidity("custom error");
  assert_equals(output.validationMessage, "", "should not have a validation message");
  assert_true(output.validity.customError, "should have a custom error");
  assert_false(output.validity.valid, "should not be valid with a custom error");
  assert_false(output.matches(":valid"), "should still not match :valid pseudo-class");
  assert_false(output.matches(":invalid"), "should still not match :invalid pseudo-class");
}, ":valid and :invalid pseudo-class on output element")

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
  "source_name": "html/semantics/forms/the-output-element/output-validity.html"
}
```
