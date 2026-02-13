# html/semantics/forms/the-input-element/show-picker-disabled-readonly.html

Counts:
- errors: 3
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-input-element/show-picker-disabled-readonly.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta name=timeout content=long>
<title>Test showPicker() disabled/readonly requirement</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<body></body>
<script type=module>
import inputTypes from "./input-types.js";

for (const inputType of inputTypes) {
  test(() => {
    const input = document.createElement("input");
    input.setAttribute("type", inputType);
    input.setAttribute("disabled", "");

    assert_throws_dom('InvalidStateError', () => { input.showPicker(); });
  }, `input[type=${inputType}] showPicker() throws when disabled`);
}

const noReadonlySupport = ['button', 'checkbox', 'color', 'file',
'hidden', 'image', 'radio', 'range', 'reset', 'submit'];
for (const inputType of inputTypes) {
  if (!noReadonlySupport.includes(inputType)) {
    promise_test(async () => {
      const input = document.createElement("input");
      input.setAttribute("type", inputType);
      input.setAttribute("readonly", "");

      await test_driver.bless('show picker');
      assert_throws_dom('InvalidStateError', () => { input.showPicker(); });

      assert_true(navigator.userActivation.isActive, 'User activation is not consumed for readonly showPicker() call');
    }, `input[type=${inputType}] showPicker() throws when readonly`);
  } else {
    promise_test(async () => {
      const input = document.createElement("input");
      input.setAttribute("type", inputType);
      input.setAttribute("readonly", "");
      document.body.appendChild(input);

      await test_driver.bless('show picker');
      input.showPicker();
      input.blur();
      input.remove();

      assert_false(navigator.userActivation.isActive, 'User activation is consumed for non-readonly showPicker() call');
    }, `input[type=${inputType}] showPicker() doesn't throw when readonly`);
  }
}
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.stray_start_tag",
      "message": "Stray start tag “script”.",
      "severity": "Error",
      "span": {
        "byte_end": 357,
        "byte_start": 337,
        "col": 1,
        "line": 9
      }
    },
    {
      "category": "Html",
      "code": "html.parser.nonspace_after_body",
      "message": "Non-space character after body.",
      "severity": "Error",
      "span": {
        "byte_end": 1973,
        "byte_start": 357,
        "col": 21,
        "line": 9
      }
    },
    {
      "category": "Html",
      "code": "html.parser.end_tag_after_body_closed",
      "message": "Saw an end tag after “body” had been closed.",
      "severity": "Error",
      "span": {
        "byte_end": 1982,
        "byte_start": 1973,
        "col": 1,
        "line": 52
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
  "source_name": "html/semantics/forms/the-input-element/show-picker-disabled-readonly.html"
}
```
