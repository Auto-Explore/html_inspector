# html/semantics/forms/the-select-element/customizable-select/select-keyboard-focus-change-for-hidden-options.optional.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/customizable-select/select-keyboard-focus-change-for-hidden-options.optional.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta name="timeout" content="long">
<link rel="help" href="http://crbug.com/377620848">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>

<!-- This test is optional because the HTML spec does not require that specific
  behaviors are mapped to specific keyboard buttons. -->

<style>
  select, select::picker(select) {
    appearance: base-select;
  }
</style>

<select id=target>
  <option id=alpha>alpha</option>
  <option hidden>bravo</option>
  <option id=charlie>charlie</option>
  <option hidden>delta</option>
  <option hidden>echo</option>
  <option id=foxtrot>foxtrot</option>
  <option hidden>golf</option>
</select>

<script>
const Space = ' ';
const ArrowUp = '\uE013';
const ArrowDown = '\uE015';

promise_test(async (t) => {
  assert_false(target.matches(':open'));
  assert_equals(target.value,'alpha','Initial select value should be alpha.');
  target.focus();
  assert_equals(document.activeElement,target,'The select should be focused.');


  await test_driver.send_keys(document.activeElement, Space);
  assert_true(target.matches(':open'),'The select should be open after pressing space.');
  assert_equals(document.activeElement,alpha,'The `alpha` option should be initially focused.');

  // ArrowDown tests
  await test_driver.send_keys(document.activeElement, ArrowDown);
  assert_equals(document.activeElement,charlie,'The `charlie` option should be focused after pressing `ArrowDown`.');
  await test_driver.send_keys(document.activeElement, ArrowDown);
  assert_equals(document.activeElement,foxtrot,'The `foxtrot` option should be focused after pressing `ArrowDown`.');
  await test_driver.send_keys(document.activeElement, ArrowDown);
  assert_equals(document.activeElement,foxtrot,'The `foxtrot` option should still be focused after pressing `ArrowDown`.');

  // ArrowUp tests
  await test_driver.send_keys(document.activeElement, ArrowUp);
  assert_equals(document.activeElement,charlie,'The `charlie` option should be focused after pressing `ArrowUp`.');
  await test_driver.send_keys(document.activeElement, ArrowUp);
  assert_equals(document.activeElement,alpha,'The `alpha` option should be focused after pressing `ArrowUp`.');
  await test_driver.send_keys(document.activeElement, ArrowUp);
  assert_equals(document.activeElement,alpha,'The `alpha` option should still be focused after pressing `ArrowUp`.');
}, 'Hidden options should be skipped when changing focus using the up and down keys.');
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
  "source_name": "html/semantics/forms/the-select-element/customizable-select/select-keyboard-focus-change-for-hidden-options.optional.html"
}
```
