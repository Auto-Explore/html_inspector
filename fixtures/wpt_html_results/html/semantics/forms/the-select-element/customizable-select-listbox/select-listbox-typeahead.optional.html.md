# html/semantics/forms/the-select-element/customizable-select-listbox/select-listbox-typeahead.optional.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/customizable-select-listbox/select-listbox-typeahead.optional.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=help href="https://github.com/openui/open-ui/issues/113#issuecomment-2845374108">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="/resources/testdriver-actions.js"></script>

<!-- This test is optional because the HTML spec does not
  define typeahead behavior. -->

<select size=4>
  <option class=one>one</option>
  <option>gap 1</option>
  <option class=two>two</option>
  <option>gap 2</option>
  <option class=three>three</option>
</select>

<select multiple size=4>
  <option class=one>one</option>
  <option>gap 1</option>
  <option class=two>two</option>
  <option>gap 2</option>
  <option class=three>three</option>
</select>

<style>
select {
  appearance: base-select
}
</style>

<script>
function pressKey(key) {
  return (new test_driver.Actions()
    .keyDown(key)
    .keyUp(key))
    .send();
}

document.querySelectorAll('select').forEach(select => {
  promise_test(async () => {
    assert_equals(getComputedStyle(select).appearance, 'base-select',
      'appearance: base-select must be supported to run this test.');
    assert_equals(select.selectedOptions.length, 0,
      'No options should be selected at the start of the test.');

    const optionOne = select.querySelector('.one');
    const optionTwo = select.querySelector('.two');
    const optionThree = select.querySelector('.three');

    optionOne.focus();
    await pressKey('t');
    assert_equals(document.activeElement, optionTwo,
      'Pressing "t" should move focus to option "two".');
    assert_equals(select.selectedOptions.length, 0,
      'No options should be selected after typeahed to option two.');

    await pressKey('h');
    assert_equals(document.activeElement, optionThree,
      'Pressing "th" should move focus to option "three".');
    assert_equals(select.selectedOptions.length, 0,
      'No options should be selected after typeahed to option three.');
  }, `<select${select.multiple ? ' multiple' : ''}>: Typeahead should focus options without checking them.`);
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
        "byte_end": 897,
        "byte_start": 890,
        "col": 1,
        "line": 29
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
  "source_name": "html/semantics/forms/the-select-element/customizable-select-listbox/select-listbox-typeahead.optional.html"
}
```
