# html/semantics/forms/customizable-combobox/active-option-pseudo.tentative.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/customizable-combobox/active-option-pseudo.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=help href="https://open-ui.org/components/combobox.explainer/">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<style>
input, datalist {
  appearance: base;
}
</style>

<input list=datalist>
<datalist id=datalist>
  <option class=one>one</option>
  <option class=two>two</option>
</datalist>

<script>
const input = document.querySelector('input');
const datalist = document.querySelector('datalist');
const optionOne = document.querySelector('.one');
const optionTwo = document.querySelector('.two');

test(() => {
  assert_false(datalist.matches(':popover-open'),
    'datalist should not match :popover-open at the start of the test.');
  assert_false(optionOne.matches(':active-option'),
    'option one should not match :active-option when the datalist is closed.');
  assert_false(optionTwo.matches(':active-option'),
    'option two should not match :active-option when the datalist is closed.');
  input.focus();
  assert_true(datalist.matches(':popover-open'),
    'datalist should open after focusing the input.');
  assert_true(optionOne.matches(':active-option'),
    'first option should match :active-option after opening the datalist.');
  assert_false(optionTwo.matches(':active-option'),
    'second option should not match :active-option after opening the datalist.');
  input.blur();
}, 'When the datalist opens, the first option should match :active-option.');

test(() => {
  assert_false(datalist.matches(':popover-open'),
    'datalist should be closed at the start of the test.');
  optionOne.setAttribute('disabled', '');
  input.focus();
  assert_true(datalist.matches(':popover-open'),
    'datalist should open after focusing the input.');
  assert_false(optionOne.matches(':active-option'),
    'disabled option should not match :active-option.');
  assert_true(optionTwo.matches(':active-option'),
    'enabled option should match :active-option.');
  input.blur();
}, 'Disabled options should not match :active-option.');
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
  "source_name": "html/semantics/forms/customizable-combobox/active-option-pseudo.tentative.html"
}
```
