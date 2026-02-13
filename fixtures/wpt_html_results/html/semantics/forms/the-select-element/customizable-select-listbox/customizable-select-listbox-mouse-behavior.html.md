# html/semantics/forms/the-select-element/customizable-select-listbox/customizable-select-listbox-mouse-behavior.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/customizable-select-listbox/customizable-select-listbox-mouse-behavior.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=help href="https://issues.chromium.org/issues/357649033">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="/resources/testdriver-actions.js"></script>

<style>
select {
  appearance: base-select;
}
</style>

<select multiple>
  <option class=one>one</option>
  <option class=two>two</option>
  <option class=disabled disabled>disabled</option>
</select>

<select size=4>
  <option class=one>one</option>
  <option class=two>two</option>
  <option class=disabled disabled>disabled</option>
</select>

<script>
function click(element) {
  return (new test_driver.Actions()
    .pointerMove(1, 1, {origin: element})
    .pointerDown()
    .pointerUp())
    .send();
}

function touch(element) {
  return (new test_driver.Actions()
    .addPointer('finger', 'touch')
    .pointerMove(1, 1, {origin: element, sourceName: 'finger'})
    .pointerDown({sourceName: 'finger'})
    .pointerUp({sourceName: 'finger'}))
    .send();
}

['mouse', 'touch'].forEach(inputType => {
  const activate = inputType == 'touch' ? touch : click;

  promise_test(async () => {
    const select = document.querySelector('select[multiple]');
    select.value = null;
    select.removeAttribute('disabled');
    const optionOne = select.querySelector('.one');
    const optionTwo = select.querySelector('.two');
    const disabledOption = select.querySelector('.disabled');
    assert_equals(select.selectedOptions.length, 0,
      'No options should be selected at the start of the test.');

    await activate(optionOne);
    assert_equals(select.selectedOptions.length, 1,
      'There should be one selected option after first click.');
    assert_equals(select.selectedOptions[0], optionOne,
      'Option one should be selected after first click.');

    await activate(optionTwo);
    assert_equals(select.selectedOptions.length, 2,
      'There should be two selected options after second click.');
    assert_equals(select.selectedOptions[0], optionOne,
      'Option one should stay selected after second click.');
    assert_equals(select.selectedOptions[1], optionTwo,
      'Option two should be seleted after second click.');

    await activate(optionTwo);
    assert_equals(select.selectedOptions.length, 1,
      'There should be one selected option after third click.');
    assert_equals(select.selectedOptions[0], optionOne,
      'Option one should stay selected after third click.');

    await activate(disabledOption);
    assert_equals(select.selectedOptions.length, 1,
      'Disabled option should not be checkable.');
    assert_equals(select.selectedOptions[0], optionOne,
      'Disabled option should not be checkable.');

    select.setAttribute('disabled', '');
    await activate(optionTwo);
    assert_equals(select.selectedOptions.length, 1,
      'Disabled select should not have checkable options.');
    assert_equals(select.selectedOptions[0], optionOne,
      'Disabled select should not have checkable options.');
  }, `${inputType}: input behavior for base appearance <select multiple>`);

  promise_test(async () => {
    const select = document.querySelector('select[size]');
    select.value = null;
    select.removeAttribute('disabled');
    const optionOne = select.querySelector('.one');
    const optionTwo = select.querySelector('.two');
    const disabledOption = select.querySelector('.disabled');
    assert_equals(select.selectedOptions.length, 0,
      'No options should be selected at the start of the test.');

    await activate(optionOne);
    assert_equals(select.selectedOptions.length, 1,
      'There should be one selected option after first click.');
    assert_equals(select.selectedOptions[0], optionOne,
      'Option one should be selected after first click.');

    await activate(optionTwo);
    assert_equals(select.selectedOptions.length, 1,
      'There should be one selected option after second click.');
    assert_equals(select.selectedOptions[0], optionTwo,
      'Option two should be selected after second click.');

    await activate(optionTwo);
    assert_equals(select.selectedOptions.length, 0,
      'There should be no selected options after third click.');

    await activate(disabledOption);
    assert_equals(select.selectedOptions.length, 0,
      'Disabled option should not be checkable.');

    select.setAttribute('disabled', '');
    await activate(optionTwo);
    assert_equals(select.selectedOptions.length, 0,
      'Disabled select should not have checkable options.');
  }, `${inputType}: input behavior for base appearance <select size=4>`);
});
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
  "source_name": "html/semantics/forms/the-select-element/customizable-select-listbox/customizable-select-listbox-mouse-behavior.html"
}
```
