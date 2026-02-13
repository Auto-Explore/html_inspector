# html/semantics/forms/the-select-element/customizable-select/select-accessibility-minimum-target-size.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/customizable-select/select-accessibility-minimum-target-size.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=help href="https://www.w3.org/WAI/WCAG22/Understanding/target-size-minimum.html">
<link rel=help href="https://github.com/openui/open-ui/issues/1026">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>

<style>
select, select::picker(select) {
  appearance: base-select;
}
</style>

<select>
  <option></option>
</select>

<script>
const option = document.querySelector('option');
for (const text of ['', 'i']) {
  for (const writingMode of ['horizontal-tb', 'vertical-lr', 'vertical-rl']) {
    promise_test(async () => {
      document.documentElement.style.writingMode = writingMode;
      option.textContent = text;
      await new Promise(requestAnimationFrame);

      const select = document.querySelector('select');
      const buttonRect = select.getBoundingClientRect();
      assert_greater_than_equal(buttonRect.width, 24,
        'Button width must be at least 24 pixels.');
      assert_greater_than_equal(buttonRect.height, 24,
        'Button height must be at least 24 pixels.');

      await test_driver.bless();
      select.showPicker();
      const optionRect = document.querySelector('option').getBoundingClientRect();
      assert_greater_than_equal(optionRect.width, 24,
        'Option width must be at least 24 pixels.');
      assert_greater_than_equal(optionRect.height, 24,
        'Option height must be at least 24 pixels.');
    }, `${writingMode} "${text}": Select with appearance:base-select should have targets for pointer inputs be at least 24x24 CSS pixels.`);
  }
}
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.option.empty_without_label",
      "message": "Element “option” without attribute “label” must not be empty.",
      "severity": "Warning",
      "span": {
        "byte_end": 549,
        "byte_start": 540,
        "col": 11,
        "line": 17
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
  "source_name": "html/semantics/forms/the-select-element/customizable-select/select-accessibility-minimum-target-size.html"
}
```
