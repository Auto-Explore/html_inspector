# html/semantics/forms/the-select-element/customizable-select/uses-label-dynamic.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/customizable-select/uses-label-dynamic.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<html class="reftest-wait">
<link rel="match" href="uses-label-dynamic-ref.html">
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<style>
select, select::picker(select) {
  appearance: base-select;
}

</style>

<select>
  <option selected id="o">Wrong label (when the test is done)</option>
  <option>Two</option>
  <option>Three</option>
</select>

<script>

document.documentElement.addEventListener("TestRendered", (ev) => {
  let option = document.getElementById("o");
  option.label = "Correct label";
  test_driver.bless('show picker', () => {
    option.parentNode.showPicker();
    document.documentElement.classList.remove("reftest-wait");
  });
}, { once: true });

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
  "source_name": "html/semantics/forms/the-select-element/customizable-select/uses-label-dynamic.html"
}
```
