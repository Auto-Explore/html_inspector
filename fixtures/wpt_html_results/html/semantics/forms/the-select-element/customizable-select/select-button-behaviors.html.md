# html/semantics/forms/the-select-element/customizable-select/select-button-behaviors.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/customizable-select/select-button-behaviors.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=help href="https://chromium-review.googlesource.com/c/chromium/src/+/6004443">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>

<style>
select,::picker(select) {
  appearance: base-select;
}
</style>

<form>
  <select>
    <button id=btninselect>button in select</button>
    <option>option</option>
  </select>
  <button id=btninform>button in form</button>
</form>

<script>
const select = document.querySelector('select');
const btninselect = document.getElementById('btninselect');
const btninform = document.getElementById('btninform');
const form = document.querySelector('form');

promise_test(async () => {
  assert_false(btninselect.matches(':default'),
    'Button in select should not match :default.');
  assert_true(btninform.matches(':default'),
    'Button in form should match :default.');


  let formWasSubmitted = false;
  form.addEventListener('submit', event => {
    event.preventDefault();
    formWasSubmitted = true;
  }, {once: true});

  await test_driver.click(select);
  assert_false(formWasSubmitted,
    'Clicking the select button should not submit the form.');

  await test_driver.click(btninform);
  assert_true(formWasSubmitted,
    'Clicking the button in the form should submit the form.');
}, 'Select button should not be the default form submit button.');

promise_test(async () => {
  select.setAttribute('disabled', '');
  assert_false(btninselect.matches(':disabled'));
}, 'Select button should not inherit :disabled from select.');
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
  "source_name": "html/semantics/forms/the-select-element/customizable-select/select-button-behaviors.html"
}
```
