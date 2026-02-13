# html/semantics/forms/the-select-element/customizable-select/select-iterate-before-beginning.optional.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/customizable-select/select-iterate-before-beginning.optional.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=help href="https://issues.chromium.org/issues/388299752">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>

<!-- This test is optional because the HTML spec does not require that specific
  behaviors are mapped to specific keyboard buttons. -->

<style>
select, ::picker(select) {
  appearance: base-select;
}
</style>

<select id=s1>
  <button>button</button>
  <option class=one>one</option>
  <option>two</option>
</select>

<select id=s2>
  <option class=one>one</option>
  <hr>
  <optgroup></optgroup>
  <option class=two>two</option>
</select>

<script>
const arrowUp = '\uE013';
const arrowDown = '\uE015';

promise_test(async () => {
  const select = document.getElementById('s1');
  const firstOption = select.querySelector('option.one');
  assert_equals(getComputedStyle(select).appearance, 'base-select',
    'appearance:base-select must be supported in order to run this test.');

  await test_driver.click(select);
  assert_true(select.matches(':open'),
    'select should open after clicking it.');
  assert_equals(document.activeElement, firstOption,
    'The first option should be focused after opening the select.');

  for (let i = 0; i < 3; i++) {
    await test_driver.send_keys(firstOption, arrowUp);
    assert_equals(document.activeElement, firstOption,
      'The first option should remain focused after pressing ArrowUp.');
  }

  // close select for the next test
  await test_driver.click(select);
  assert_false(select.matches(':open'),
    'select should be closed at the end of the test.');
}, 'Attempting to focus the previous option while focused on the first option should not crash.');

promise_test(async () => {
  const select = document.getElementById('s2');
  const firstOption = select.querySelector('option.one');
  const secondOption = select.querySelector('option.two');
  assert_equals(getComputedStyle(select).appearance, 'base-select',
    'appearance:base-select must be supported in order to run this test.');

  await test_driver.click(select);
  assert_true(select.matches(':open'),
    'select should open after clicking it.');

  assert_equals(document.activeElement, firstOption,
    'first option should be focused after opening select.');

  await test_driver.send_keys(document.activeElement, arrowDown);
  assert_equals(document.activeElement, secondOption,
    'second option should be focused after arrow down.');

  await test_driver.send_keys(document.activeElement, arrowUp);
  assert_equals(document.activeElement, firstOption,
    'first option should be focused after arrow up.');

  await test_driver.click(select);
  assert_false(select.matches(':open'),
    'select should be closed at the end of the test.');
}, 'Keyboard navigating backwards over an <hr> and <optgroup> should not crash.');
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
  "source_name": "html/semantics/forms/the-select-element/customizable-select/select-iterate-before-beginning.optional.html"
}
```
