# html/semantics/forms/the-select-element/customizable-select/button-in-popover.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/customizable-select/button-in-popover.html",
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

<style>
select, select::picker(select) {
  appearance: base-select;
}
</style>

<select>
  <button id=invoker>invoker</button>
  <option id=option1>one</option>
  <option>two</option>
  <button id=popover>popover button</button>
  <span id=other>other text</span>
</select>

<script>
const select = document.querySelector('select');
const option1 = document.getElementById('option1');
const popoverButton = document.getElementById('popover');
const otherContent = document.getElementById('other');

function assertAppearance() {
  assert_equals(getComputedStyle(select).appearance, 'base-select',
    'appearance:base-select must be supported in order to run this test.');
}

promise_test(async () => {
  assertAppearance();
  assert_false(select.matches(':open'),
    'Select should be closed at the start of the test.');
  await test_driver.click(select);
  assert_true(select.matches(':open'),
    'Select should open after clicking the invoker button.');

  let popoverButtonClicked = false;
  popoverButton.onclick = () => popoverButtonClicked = true;
  await test_driver.click(popoverButton);
  assert_true(select.matches(':open'),
    'Clicking the button should not have closed the popover.');
  assert_true(popoverButtonClicked,
    'The button in the popover should have gotten a click event when clicked.');

  popoverButton.focus();
  const ENTER_KEY = '\uE007';
  await test_driver.send_keys(popoverButton, ENTER_KEY);
  assert_true(select.matches(':open'),
    'Keyboard-activating the button should also not have closed the popover.');

  await test_driver.click(select);
  assert_false(select.matches(':open'),
    'Clicking invoker button should close select.');
}, 'Buttons in the popover should be rendered and should not close the popover when clicked.');

promise_test(async () => {
  assertAppearance();
  assert_false(select.matches(':open'),
    'Select should be closed at the start of the test.');
  await test_driver.click(select);
  assert_true(select.matches(':open'),
    'Select should open after clicking it.');

  await test_driver.click(other);
  assert_true(select.matches(':open'),
    'Clicking non-interactive, non-option content should not close the popover.');

  await test_driver.click(select);
  assert_false(select.matches(':open'),
    'Clicking invoker button should close select.');
}, 'Non-interactive content in the popover should not close the popover when clicked.');
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
  "source_name": "html/semantics/forms/the-select-element/customizable-select/button-in-popover.html"
}
```
