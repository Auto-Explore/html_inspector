# html/semantics/forms/the-select-element/customizable-select/select-home-end-pagedown-pageup.optional.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/customizable-select/select-home-end-pagedown-pageup.optional.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta name=timeout content=long>
<link rel=author href="mailto:masonf@chromium.org">
<link rel=help href="https://issues.chromium.org/issues/382101095">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>

<!-- This test is optional because the HTML spec does not require that specific
  behaviors are mapped to specific keyboard buttons. -->

<style>
select,::picker(select) {
  appearance: base-select;
}
</style>

<select></select>

<script>
const keyMap = {
  'Home': '\uE011',
  'End': '\uE010',
  'PageUp': '\uE00E',
  'PageDown': '\uE00F'
};
const select = document.querySelector('select');

for(let i=1;i<=1000;++i) {
  const option = document.createElement('option');
  option.textContent = `Option #${i}`;
  option.id=i;
  select.appendChild(option);
}
const firstOption = document.getElementById(1);
const middleOption = document.getElementById(500);

['Home', 'End', 'PageUp', 'PageDown'].forEach(keyName => {
  promise_test(async () => {
    assert_false(select.matches(':open'));
    select.value = middleOption.value;
    assert_equals(select.value, middleOption.value,'Initial value');
    await test_driver.click(select);
    assert_true(select.matches(':open'));
    assert_equals(select.value, middleOption.value,'Value doesn\'t change when opening picker');
    assert_equals(document.activeElement, middleOption, 'Selected option should be focused');

    const keyCode = keyMap[keyName];
    await test_driver.send_keys(document.activeElement, keyCode);
    assert_equals(select.value, middleOption.value, 'Selected option should not change');
    assert_not_equals(document.activeElement, middleOption, 'Focus should move');

    select.value = firstOption.value;
    await test_driver.click(select);
    assert_false(select.matches(':open'),'Clicking select should close picker');
    assert_equals(select.value, firstOption.value, 'Value should stay');
  }, `Behavior of ${keyName} for customizable-<select>`);
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
  "source_name": "html/semantics/forms/the-select-element/customizable-select/select-home-end-pagedown-pageup.optional.html"
}
```
