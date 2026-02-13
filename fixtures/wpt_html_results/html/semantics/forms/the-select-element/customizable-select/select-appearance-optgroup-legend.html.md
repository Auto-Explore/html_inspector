# html/semantics/forms/the-select-element/customizable-select/select-appearance-optgroup-legend.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/customizable-select/select-appearance-optgroup-legend.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html class=reftest-wait>
<meta name=fuzzy content="maxDifference=0-3;totalPixels=0-1">
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=help href="https://github.com/whatwg/html/issues/9799">
<link rel=match href="select-appearance-optgroup-legend-ref.html">
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>

<style>
select, select::picker(select) {
  appearance: base-select;
}
</style>

<select>
  <option>one</option>
  <optgroup>
    <legend>legend</legend>
    <option>two</option>
  </optgroup>
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
  "source_name": "html/semantics/forms/the-select-element/customizable-select/select-appearance-optgroup-legend.html"
}
```
