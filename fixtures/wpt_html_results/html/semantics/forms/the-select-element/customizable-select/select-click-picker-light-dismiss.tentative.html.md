# html/semantics/forms/the-select-element/customizable-select/select-click-picker-light-dismiss.tentative.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/customizable-select/select-click-picker-light-dismiss.tentative.html",
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

<style>
select, ::picker(select) {
  appearance: base-select;
}
::picker(select) {
  width: 600px;
  height: 600px;
  top: 0;
  left: 0;
  position-area: none;
  position-try-fallbacks: none;
}
</style>

<select></select>

<script>
promise_test(async () => {
  const select = document.querySelector('select');
  await (new test_driver.Actions()
    .pointerMove(0, 0, {origin: select})
    .pointerDown()
    .pointerUp())
    .send();
  await new Promise(requestAnimationFrame);
  assert_true(select.matches(':open'),
    'Select should be open after clicking it.');

  let selectClicked = false;
  select.addEventListener('click', () => {
    selectClicked = true;
  });

  await (new test_driver.Actions()
    .pointerMove(300, 300)
    .pointerDown()
    .pointerUp())
    .send();

  assert_true(selectClicked,
    'Clicking ::picker(select) should fire a click event on the select element.');
  assert_true(select.matches(':open'),
    'Select should not close when clicking its picker.');
}, 'Clicking directly on ::picker(select) should not trigger light dismiss.');
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
  "source_name": "html/semantics/forms/the-select-element/customizable-select/select-click-picker-light-dismiss.tentative.html"
}
```
