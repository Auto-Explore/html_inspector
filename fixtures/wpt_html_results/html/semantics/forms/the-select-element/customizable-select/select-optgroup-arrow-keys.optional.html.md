# html/semantics/forms/the-select-element/customizable-select/select-optgroup-arrow-keys.optional.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/customizable-select/select-optgroup-arrow-keys.optional.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=help href="https://issues.chromium.org/issues/417119055">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="/resources/testdriver-actions.js"></script>

<!-- This test is optional because the HTML spec does not require that specific
  behaviors are mapped to specific keyboard buttons. -->

<style>
select, ::picker(select) {
  appearance: base-select;
}
</style>

<select>
  <optgroup label=group1>
    <option class=one>one</option>
    <option class=two>two</option>
  </optgroup>
  <optgroup label=group2>
    <option class=three>three</option>
    <option class=four>four</option>
  </optgroup>
  <optgroup label=group3>
    <option class=five>five</option>
    <option class=six>six</option>
  </optgroup>
</select>

<script>
const ArrowUp = '\uE013';
const ArrowDown = '\uE015';
const Space = ' ';

function sendKey(key) {
  return (new test_driver.Actions()
    .keyDown(key)
    .keyUp(key))
    .send();
}

promise_test(async () => {
  const select = document.querySelector('select');
  const options = [
    document.querySelector('.one'),
    document.querySelector('.two'),
    document.querySelector('.three'),
    document.querySelector('.four'),
    document.querySelector('.five'),
    document.querySelector('.six')
  ];

  assert_equals(getComputedStyle(select).appearance, 'base-select',
    'appearance:base-select must be supported to run this test.');

  select.focus();
  await sendKey(Space);
  assert_true(select.matches(':open'),
    'Space should open picker.');
  assert_equals(document.activeElement, options[0],
    'First option should be initially focused.');

  for (let i = 1; i < 6; i++) {
    await sendKey(ArrowDown);
    assert_equals(document.activeElement, options[i],
      `Option ${i} should be focused after ArrowDown.`);
  }

  for (let i = 4; i > -1; i--) {
    await sendKey(ArrowUp);
    assert_equals(document.activeElement, options[i],
      `Option ${i} should be focused after ArrowUp.`);
  }
}, 'Keyboard navigation forwards and backwards should visit each option with optgroups.');
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
  "source_name": "html/semantics/forms/the-select-element/customizable-select/select-optgroup-arrow-keys.optional.html"
}
```
