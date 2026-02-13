# html/semantics/forms/customizable-combobox/customizable-combobox-appearance.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/customizable-combobox/customizable-combobox-appearance.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=help href="https://open-ui.org/components/combobox.explainer/">
<link rel=match href="customizable-combobox-appearance-ref.html">

<style>
input, datalist {
  appearance: base;
}
input {
  width: 150px;
}
</style>

<input list=datalist>
<datalist id=datalist>
  <option>one</option>
  <option>two</option>
</datalist>

<script>
document.querySelector('input').focus();
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
  "source_name": "html/semantics/forms/customizable-combobox/customizable-combobox-appearance.html"
}
```
