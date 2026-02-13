# html/semantics/forms/the-select-element/customizable-select-multiple-popup/select-multiple-popup-keyboard-behavior.tentative.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/customizable-select-multiple-popup/select-multiple-popup-keyboard-behavior.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=help href="https://github.com/openui/open-ui/issues/1102">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="/resources/testdriver-actions.js"></script>

<style>
select, ::picker(select) {
  appearance: base-select;
}
</style>

<select multiple size=1>
  <option class=one>one</option>
  <option class=two>two</option>
</select>

<script>
const select = document.querySelector('select');
const optionOne = select.querySelector('.one');
const optionTwo = select.querySelector('.two');
const tabKey = '\uE004';
const spaceKey = '\uE00D';
const arrowDown = '\uE015';
const arrowUp = '\uE013';
const escapeKey = '\uE00C';

function pressKey(key) {
  return (new test_driver.Actions()
    .keyDown(key)
    .keyUp(key))
    .send();
}

promise_test(async () => {
  assert_equals(getComputedStyle(select).appearance, 'base-select',
    'appearance:base-select must be supported to run this test.');
  assert_false(select.matches(':open'),
    'Select should be closed at the start of the test.');
  assert_equals(select.selectedOptions.length, 0,
    'No options should be selected at the start of the test.');
  assert_false(optionOne.matches(':checked'),
    'Option one should not be checked at the start of the test.');
  assert_false(optionTwo.matches(':checked'),
    'Option two should not be checked at the start of the test.');

  select.focus();
  await pressKey(spaceKey);
  assert_true(select.matches(':open'),
    'Select should be open after pressing space.');
  assert_equals(document.activeElement, optionOne,
    'Option one should be focused when opening picker.');

  await pressKey(spaceKey);
  assert_true(select.matches(':open'),
    'The picker should stay open after pressing space on the first option.');
  assert_equals(document.activeElement, optionOne,
    'Option one should stay focused after pressing space on it.');
  assert_true(optionOne.matches(':checked'),
    'Option one should match :checked after pressing space on it.');
  assert_false(optionTwo.matches(':checked'),
    'Option two should not match :checked after pressing space on option one.');
  assert_equals(select.selectedOptions.length, 1,
    'There should be one selected option after pressing space on the first option.');
  assert_equals(select.selectedOptions[0], optionOne,
    'Option one should be the selected option after pressing space on it.');

  await pressKey(arrowDown);
  assert_equals(document.activeElement, optionTwo,
    'Option two should become focused after arrow down on option one.');

  await pressKey(spaceKey);
  assert_true(select.matches(':open'),
    'The picker should stay open after pressing space on the second option.');
  assert_true(optionOne.matches(':checked'),
    'Option one should still match :checked after pressing space on option two.');
  assert_true(optionTwo.matches(':checked'),
    'Option two should match :checked after pressing space on it.');
  assert_equals(select.selectedOptions.length, 2,
    'There should be two selected options after pressing space on the second option.');
  assert_equals(select.selectedOptions[1], optionTwo,
    'Option two should be the second selected option after pressing space on it.');

  await pressKey(spaceKey);
  assert_true(select.matches(':open'),
    'The picker should stay open after pressing space no the second option twice.');
  assert_true(optionOne.matches(':checked'),
    'Option one should still match :checked after pressing space on the second option twice.');
  assert_false(optionTwo.matches(':checked'),
    'Option two should not match :checked after pressing space on it twice.');
  assert_equals(select.selectedOptions.length, 1,
    'There should be one selected option after pressing space on the second option twice.');

  await pressKey(escapeKey);
  assert_false(select.matches(':open'),
    'Picker should close after pressing escape key.');
}, 'Keyboard behavior for base appearance <select multiple size=1>');
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
  "source_name": "html/semantics/forms/the-select-element/customizable-select-multiple-popup/select-multiple-popup-keyboard-behavior.tentative.html"
}
```
