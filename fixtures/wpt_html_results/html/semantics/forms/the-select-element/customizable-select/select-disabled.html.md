# html/semantics/forms/the-select-element/customizable-select/select-disabled.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/customizable-select/select-disabled.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=help href="https://issues.chromium.org/issues/40265812">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>

<select id=defaultbutton disabled style="appearance:base-select">
  <option>one</option>
  <option>two</option>
</select>

<select id=custombutton disabled style="appearance:base-select">
  <button>button</button>
  <option>one</option>
  <option>two</option>
</select>

<script>
['defaultbutton', 'custombutton'].forEach(id => {
  promise_test(async () => {
    assert_true(CSS.supports('appearance', 'base-select'),
      'This test requires appearance:base-select in order to run.');

    const select = document.getElementById(id);
    select.focus();
    assert_not_equals(document.activeElement, select,
      'select should not be focusable when disabled.');

    await test_driver.click(select);
    assert_false(select.matches(':open'),
      'select should not be open after clicking when disabled.');

    const button = select.querySelector('button');
    if (button) {
      button.focus();
      assert_not_equals(document.activeElement, button,
        'select button should not be focusable when select is disabled.');
    }
  }, `${id}: <select disabled> should prevent focus and activation with appearance:base-select.`);
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
  "source_name": "html/semantics/forms/the-select-element/customizable-select/select-disabled.html"
}
```
