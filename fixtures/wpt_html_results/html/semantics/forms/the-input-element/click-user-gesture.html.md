# html/semantics/forms/the-input-element/click-user-gesture.html

Counts:
- errors: 3
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-input-element/click-user-gesture.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Test click() user gesture for pickers</title>
<meta name="timeout" content="long">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<body></body>
<script type=module>
import inputTypes from "./input-types.js";

const pickerTypes = ['color', 'date', 'datetime-local', 'file', 'month', 'time', 'week'];

for (const inputType of pickerTypes) {
  promise_test(async t => {
    const input = document.createElement("input");
    input.setAttribute("type", inputType);

    await test_driver.bless('click');
    input.click();
    input.blur();
  }, `input[type=${inputType}] click() does not throw when user activation is active`);
}

for (const inputType of pickerTypes) {
  promise_test(async () => {
    const input = document.createElement('input');
    input.setAttribute('type', inputType);

    await test_driver.bless('click');
    input.click();
    input.blur();

    assert_false(navigator.userActivation.isActive);
  }, `input[type=${inputType}] click() consumes user activation`);
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
        "byte_end": 351,
        "byte_start": 331,
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
        "byte_end": 1176,
        "byte_start": 351,
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
        "byte_end": 1185,
        "byte_start": 1176,
        "col": 1,
        "line": 37
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
  "source_name": "html/semantics/forms/the-input-element/click-user-gesture.html"
}
```
