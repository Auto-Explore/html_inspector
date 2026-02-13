# html/semantics/forms/the-select-element/select-option-focusable.tentative.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/select-option-focusable.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html lang="en">
<title>select element: option focusable</title>
<link rel="author" title="Ionel Popescu" href="mailto:iopopesc@microsoft.com">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="/resources/testdriver-vendor.js"></script>

<select id="select0">
  <option>one</option>
  <option id="select0-option2">two</option>
  <option>three</option>
</select>

<style>
  select, ::picker(select) {
    appearance: base-select;
  }
</style>

<script>
function clickOn(element) {
  const actions = new test_driver.Actions();
  return actions.pointerMove(0, 0, {origin: element})
      .pointerDown({button: actions.ButtonType.LEFT})
      .pointerUp({button: actions.ButtonType.LEFT})
      .send();
}

promise_test(async t => {
  const select = document.querySelector("#select0");
  assert_false(select.matches(':open'), "select should not be initially open");

  await clickOn(select);
  assert_true(select.matches(':open'));
  assert_equals(select.value, "one");

  const option2 = document.querySelector('#select0-option2');
  option2.focus();
  assert_equals(document.activeElement, option2);

  const kEnter = '\uE007';
  await test_driver.send_keys(option2, kEnter);
  assert_equals(select.value, "two");
}, "Validate <option> is focusable when is a descendant of <select>");
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.style.not_allowed_here",
      "message": "Element “style” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 561,
        "byte_start": 554,
        "col": 1,
        "line": 17
      }
    }
  ],
  "source_name": "html/semantics/forms/the-select-element/select-option-focusable.tentative.html"
}
```
