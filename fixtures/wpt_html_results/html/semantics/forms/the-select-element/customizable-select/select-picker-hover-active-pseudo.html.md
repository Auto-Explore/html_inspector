# html/semantics/forms/the-select-element/customizable-select/select-picker-hover-active-pseudo.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/customizable-select/select-picker-hover-active-pseudo.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=help href="https://github.com/w3c/csswg-drafts/issues/11185">
<link rel=author href="mailto:jarhar@chromium.org">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="/resources/testdriver-actions.js"></script>

<select id=defaultbutton>
  <option>option</option>
</select>

<select id=custombutton>
  <button>button</button>
  <option>option</option>
</select>

<style>
select, ::picker(select) {
  appearance: base-select;
}
</style>

<script>

['defaultbutton', 'custombutton'].forEach(id => {
  const select = document.getElementById(id);
  const option = select.querySelector('option');
  function assertSupport() {
    const style = getComputedStyle(document.querySelector('select'));
    assert_equals(style.appearance, 'base-select',
      'appearance:base-select must be supported in order to run this test.');
  }

  promise_test(async () => {
    assertSupport();
    await (new test_driver.Actions()
      .pointerMove(1, 1, {origin: select}))
      .send();
    await new Promise(requestAnimationFrame);
    assert_true(select.matches(':hover'),
      'select should match :hover.');
    assert_true(document.body.matches(':hover'),
      'document.body should match :hover.');

    // sending a test_driver.Actions() which only has a pointerDown() will
    // automatically add a pointerUp(), so we have to perform an entire click
    // and check :active inside the mousedown listener instead.
    let selectMatchedActive = false;
    let bodyMatchedActive = false;
    select.addEventListener('mousedown', () => {
      selectMatchedActive = select.matches(':active');
      bodyMatchedActive = document.body.matches(':active');
    });
    await test_driver.click(select);
    assert_true(selectMatchedActive,
      'select should match :active.');
    assert_true(bodyMatchedActive,
      'document.body should match :active.');

    await test_driver.click(select);
    assert_false(select.matches(':open'),
      'select should be closed at the end of the test.');
  }, `${id}: select should match :hover and :active when interacting with button.`);

  promise_test(async () => {
    assertSupport();
    assert_false(select.matches(':open'),
      'select should be closed at the start of the test.');
    await test_driver.click(select);
    await new Promise(requestAnimationFrame);
    assert_true(select.matches(':open'),
      'select should be open after clicking it.');

    await (new test_driver.Actions()
      .pointerMove(1, 1, {origin: option}))
      .send();
    await new Promise(requestAnimationFrame);
    assert_true(option.matches(':hover'),
      'option should match :hover.');
    assert_false(select.matches(':hover'),
      'select should not match :hover.');
    assert_false(document.body.matches(':hover'),
      'document.body should not match :hover.');

    // sending a test_driver.Actions() which only has a pointerDown() will
    // automatically add a pointerUp(), so we have to perform an entire click
    // and check :active inside the mousedown listener instead.
    let optionMatchedActive = false;
    let selectMatchedActive = false;
    let bodyMatchedActive = false;
    option.addEventListener('mousedown', () => {
      optionMatchedActive = option.matches(':active');
      selectMatchedActive = select.matches(':active');
      bodyMatchedActive = document.body.matches(':active');
    });
    await (new test_driver.Actions()
      .pointerMove(1, 1, {origin: option})
      .pointerDown()
      .pointerUp())
      .send();
    assert_true(optionMatchedActive,
      'option should match :active.');
    assert_false(selectMatchedActive,
      'select should not match :active.');
    assert_false(bodyMatchedActive,
      'document.body should not match :active.');
    assert_false(select.matches(':open'),
      'select should be closed at the end of the test.');
  }, `${id}: select should not match :hover or :active when interacting with elements in the picker.`);
});
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
        "byte_end": 567,
        "byte_start": 560,
        "col": 1,
        "line": 19
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
  "source_name": "html/semantics/forms/the-select-element/customizable-select/select-picker-hover-active-pseudo.html"
}
```
