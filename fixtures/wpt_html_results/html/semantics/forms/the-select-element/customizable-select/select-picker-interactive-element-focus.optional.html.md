# html/semantics/forms/the-select-element/customizable-select/select-picker-interactive-element-focus.optional.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/customizable-select/select-picker-interactive-element-focus.optional.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="/resources/testdriver-actions.js"></script>

<!-- This test is optional because the it tests the behavior of elements not
  included in the select element's content model. -->

<select>
  <button>invoker</button>
  <button id=button>button</button>
  <option>option</option>
</select>

<style>
select, ::picker(select) {
  appearance: base-select;
}
</style>

<script>
function click(element) {
  return (new test_driver.Actions()
    .pointerMove(1, 1, {origin: element})
    .pointerDown()
    .pointerUp())
    .send();
}

promise_test(async () => {
  const select = document.querySelector('select');
  const button = document.getElementById('button');
  const input = document.createElement('input');
  select.appendChild(input);

  await click(select);
  assert_true(select.matches(':open'),
    'select should open after being clicked.');

  await click(button);
  assert_true(select.matches(':open'),
    'select should stay open after clicking button in picker.');
  assert_equals(document.activeElement, button, 'button');

  await click(input);
  assert_true(select.matches(':open'),
    'select should stay open after clicking input in picker.');
  assert_equals(document.activeElement, input, 'input');
}, 'Clicking interactive elements inside the select picker should focus them.');
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
        "byte_end": 585,
        "byte_start": 578,
        "col": 1,
        "line": 18
      }
    },
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
  "source_name": "html/semantics/forms/the-select-element/customizable-select/select-picker-interactive-element-focus.optional.html"
}
```
