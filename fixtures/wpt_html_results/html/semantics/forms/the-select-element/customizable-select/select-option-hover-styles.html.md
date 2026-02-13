# html/semantics/forms/the-select-element/customizable-select/select-option-hover-styles.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/customizable-select/select-option-hover-styles.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=help href="https://github.com/whatwg/html/issues/9799">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="/resources/testdriver-actions.js"></script>

<style>
select, select::picker(select) {
  appearance: base-select;
}
</style>

<select>
  <option class=one>one</option>
  <option class=two>two</option>
  <option class=three disabled>disabled</option>
</select>

<script>
const select = document.querySelector('select');
const optionOne = document.querySelector('.one');
const optionTwo = document.querySelector('.two');
const disabledOption = document.querySelector('.three');

promise_test(async () => {
  assert_equals(getComputedStyle(select).appearance, 'base-select',
    'appearance:base-select must be supported in order to run this test.');

  await test_driver.bless();
  select.showPicker();
  assert_true(select.matches(':open'),
    'dropdown should open after calling showPicker().');

  await (new test_driver.Actions()
    .pointerMove(1, 1, {origin: optionOne}))
    .send();
  await new Promise(requestAnimationFrame);
  assert_equals(getComputedStyle(optionOne).color, 'rgb(0, 0, 0)',
    'Option color while hovering.');
  assert_equals(getComputedStyle(optionOne).backgroundColor, 'lab(0 0 0 / 0.1)',
    'Option background-color while hovering.');

  await (new test_driver.Actions()
    .pointerMove(1, 1, {origin: disabledOption}))
    .send();
  await new Promise(requestAnimationFrame);
  assert_equals(getComputedStyle(disabledOption).color, 'lab(0 0 0 / 0.5)',
    'Disabled option color while hovering.');
  assert_equals(getComputedStyle(disabledOption).backgroundColor, 'rgba(0, 0, 0, 0)',
    'Disabled option background-color while hovering.');
}, 'Hover styles should be present for appearance:base-select options.');
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
  "source_name": "html/semantics/forms/the-select-element/customizable-select/select-option-hover-styles.html"
}
```
