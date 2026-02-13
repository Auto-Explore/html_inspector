# html/semantics/forms/the-select-element/customizable-select/select-appearance-button-after-option.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/customizable-select/select-appearance-button-after-option.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html class=reftest-wait>
<meta name=fuzzy content="maxDifference=0-100;totalPixels=0-50">
<link rel=author href="mailto:masonf@chromium.org">
<link rel=match href="select-appearance-button-after-option-ref.html">
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="resources/customizable-select-utils.js"></script>

<style>
select, ::picker(select) {
  appearance: base-select;
}
</style>

<select>
  <option>option 1</option>
  <button>Hello <selectedcontent></selectedcontent></button>
  <option>option 2</option>
</select>

<script>
clickSelectAndCaptureAppearance();
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
  "source_name": "html/semantics/forms/the-select-element/customizable-select/select-appearance-button-after-option.html"
}
```
