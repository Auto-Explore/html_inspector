# html/semantics/forms/the-input-element/show-picker-does-not-focus.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-input-element/show-picker-does-not-focus.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Test showPicker() does not focus the element</title>
<meta name="timeout" content="long">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>

<body>
<script type=module>
import inputTypes from "./input-types.js";

function createElement(t, type) {
  const input = document.createElement("input");
  input.type = type;
  document.body.append(input);
  t.add_cleanup(() => input.remove());
  return input;
}

for (const inputType of inputTypes) {
  promise_test(async (t) => {
    const input = createElement(t, inputType);
    await test_driver.bless('show picker');
    input.showPicker();
    assert_not_equals(input, document.activeElement, "Input element was not focused");
  }, `input[type=${inputType}].showPicker() does not focus the element`);
}
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
  "source_name": "html/semantics/forms/the-input-element/show-picker-does-not-focus.html"
}
```
