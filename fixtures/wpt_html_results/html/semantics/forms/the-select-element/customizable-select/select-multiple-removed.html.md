# html/semantics/forms/the-select-element/customizable-select/select-multiple-removed.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/customizable-select/select-multiple-removed.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:ansollan@microsoft.com">
<link rel=help href="https://issues.chromium.org/issues/377966015">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>

<style>
select, select::picker(select) {
  appearance: base-select;
}
</style>

<select multiple></select>

<script>
  promise_test(async () => {
    step_timeout(() => {
        document.querySelector('select').removeAttribute('multiple');
    }, 10);
  }, 'Removing the multiple attribute should not crash.');
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
  "source_name": "html/semantics/forms/the-select-element/customizable-select/select-multiple-removed.html"
}
```
