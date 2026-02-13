# html/semantics/forms/the-select-element/customizable-select/select-picker-animations.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/customizable-select/select-picker-animations.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=help href="https://github.com/whatwg/html/issues/9799">
<link rel=help href="https://issues.chromium.org/issues/359279550">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>

<style>
select, select::picker(select) {
  appearance: base-select;
}

select::picker(select) {
  transition-behavior: allow-discrete;
  transition-duration: 100s;
  transition-property: display, overlay, opacity, color;
  transition-timing-function: steps(1, jump-both);
  opacity: 0;
  color: black;
}

select::picker(select):popover-open {
  opacity: 1;
  color: rgb(200, 0, 0);
}
@starting-style {
  select::picker(select):popover-open {
    opacity: 0;
    color: black;
  }
}
</style>

<select>
  <option>one</option>
  <option>two</option>
</select>

<script>
const select = document.querySelector('select');
const firstOption = document.querySelector('option');

promise_test(async () => {
  assert_equals(document.styleSheets[0].cssRules.length, 4,
    'All 4 of the CSS rules should have been parsed.');

  assert_equals(getComputedStyle(firstOption).color, 'rgb(0, 0, 0)',
    'option color should be black before animation starts.');
  await test_driver.bless();
  select.showPicker();
  await new Promise(requestAnimationFrame);
  await new Promise(requestAnimationFrame);
  assert_equals(getComputedStyle(firstOption).color, 'rgb(100, 0, 0)',
    'option color should start animating when opening the picker.');
}, 'select::picker(select) should support author provided top layer animations.');
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
  "source_name": "html/semantics/forms/the-select-element/customizable-select/select-picker-animations.html"
}
```
