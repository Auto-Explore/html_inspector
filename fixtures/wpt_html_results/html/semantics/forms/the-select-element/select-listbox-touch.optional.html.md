# html/semantics/forms/the-select-element/select-listbox-touch.optional.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/select-listbox-touch.optional.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="/resources/testdriver-actions.js"></script>

<!-- This test is marked optional because the spec does not mention anything
  about what should happen when options are tapped with a touch screen, or any
  other activation behavior when the select is an appearance:auto listbox. -->

<select multiple>
  <option class=one>one</option>
  <option class=two>two</option>
</select>

<script>
const select = document.querySelector('select');
const optionOne = document.querySelector('option.one');
const optionTwo = document.querySelector('option.two');

function touch(target) {
  return (new test_driver.Actions()
    .addPointer('finger', 'touch')
    .pointerMove(0, 0, {origin: target})
    .pointerDown()
    .pointerUp())
    .send();
}

promise_test(async () => {
  assert_false(optionOne.selected,
    'option one should not be selected at the start of the test.');
  assert_false(optionTwo.selected,
    'option two should not be selected at the start of the test.');

  await touch(optionOne);
  assert_true(optionOne.selected,
    'option one should be selected after touching it.');
  assert_false(optionTwo.selected,
    'option two should not be selected after touching it.');

  await touch(optionTwo);
  assert_true(optionOne.selected,
    'option one should still be selected after touching option two.');
  assert_true(optionTwo.selected,
    'option two should be selected after touching it.');

  await touch(optionTwo);
  assert_true(optionOne.selected,
    'option one should still be selected after de-selecting option two.');
  assert_false(optionTwo.selected,
    'option two should be de-selected after touching it twice.');
}, 'Using touch inputs should not deselect other options in a listbox <select multiple>.');
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
  "source_name": "html/semantics/forms/the-select-element/select-listbox-touch.optional.html"
}
```
