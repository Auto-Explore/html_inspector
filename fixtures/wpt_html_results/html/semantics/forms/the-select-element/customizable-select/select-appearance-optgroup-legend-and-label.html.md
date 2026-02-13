# html/semantics/forms/the-select-element/customizable-select/select-appearance-optgroup-legend-and-label.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/customizable-select/select-appearance-optgroup-legend-and-label.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html class=reftest-wait>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=help href="https://issues.chromium.org/issues/378601807">
<link rel=match href="select-appearance-optgroup-legend-ref.html">
<meta name=assert content="The legend element should take precedence over the label attribute">
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>

<style>
select, select::picker(select) {
  appearance: base-select;
}
</style>

<select>
  <option>one</option>
  <optgroup label=label>
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
  "source_name": "html/semantics/forms/the-select-element/customizable-select/select-appearance-optgroup-legend-and-label.html"
}
```
