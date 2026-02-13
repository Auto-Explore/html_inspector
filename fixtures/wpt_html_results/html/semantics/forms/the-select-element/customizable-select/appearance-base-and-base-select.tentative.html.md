# html/semantics/forms/the-select-element/customizable-select/appearance-base-and-base-select.tentative.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/customizable-select/appearance-base-and-base-select.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html class=reftest-wait>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=help href="https://drafts.csswg.org/css-ui-4/#valdef-appearance-base-select">
<link rel=match href="appearance-base-and-base-select-ref.html">
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>

<style>
select, ::picker(select) {
  appearance: base-select;
}
</style>

<select>
  <option>one</option>
  <option>two</option>
</select>

<script>
(async () => {
  await test_driver.bless();
  document.querySelector('select').showPicker();
  document.documentElement.classList.remove('reftest-wait');
})();
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
  "source_name": "html/semantics/forms/the-select-element/customizable-select/appearance-base-and-base-select.tentative.html"
}
```
