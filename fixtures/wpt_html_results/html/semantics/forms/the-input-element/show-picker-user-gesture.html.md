# html/semantics/forms/the-input-element/show-picker-user-gesture.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-input-element/show-picker-user-gesture.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Test showPicker() user gesture requirement</title>
<meta name="timeout" content="long">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>

<body>
<script type=module>
import inputTypes from "./input-types.js";

// File pickers can't be closed.
const types = inputTypes.filter((t) => t != 'file');

function createElement(t,type) {
  const input = document.createElement("input");
  input.setAttribute("type", type);
  document.body.appendChild(input);
  t.add_cleanup(() => input.remove());
  return input;
}
for (const inputType of types) {
  promise_test(async (t) => {
    const input = createElement(t,inputType);
    assert_throws_dom('NotAllowedError', () => { input.showPicker(); });
  }, `input[type=${inputType}] showPicker() requires a user gesture`);

  promise_test(async (t) => {
    const input = createElement(t,inputType);
    await test_driver.bless('show picker');
    input.showPicker();
    input.blur();
  }, `input[type=${inputType}] showPicker() does not throw when user activation is active`);

  promise_test(async (t) => {
    const input = createElement(t,inputType);
    await test_driver.bless('show picker');
    input.showPicker();
    input.blur();
    assert_false(navigator.userActivation.isActive);
  }, `input[type=${inputType}] showPicker() consumes user activation`);
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
  "source_name": "html/semantics/forms/the-input-element/show-picker-user-gesture.html"
}
```
