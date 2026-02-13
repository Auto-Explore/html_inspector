# html/semantics/forms/customizable-combobox/datalist-popover.tentative.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/customizable-combobox/datalist-popover.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=help href="https://open-ui.org/components/combobox.explainer/">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<style>
input, datalist {
  appearance: base;
}
</style>

<input list=datalist>
<datalist id=datalist>
  <option>one</option>
  <option>two</option>
</datalist>

<script>
test(() => {
  const input = document.querySelector('input');
  const datalist = document.querySelector('datalist');
  assert_false(datalist.matches(':popover-open'),
    'datalist should not be :popover-open at the start of the test.');
  input.focus();
  assert_true(datalist.matches(':popover-open'),
    'datalist should be :popover-open after focusing the input.');
}, 'Focusing input should show datalist as a popover.');
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
  "source_name": "html/semantics/forms/customizable-combobox/datalist-popover.tentative.html"
}
```
