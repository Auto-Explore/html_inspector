# html/semantics/forms/customizable-combobox/customizable-combobox-appearance-ref.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/customizable-combobox/customizable-combobox-appearance-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=stylesheet href="resources/customizable-combobox-styles.css">

<div class=customizable-combobox-input tabindex=0></div>
<div class=customizable-combobox-datalist popover=auto>
  <div class="customizable-combobox-option active-option">one</div>
  <div class=customizable-combobox-option>two</div>
</div>

<script>
  document.querySelector('.customizable-combobox-input').focus();
  document.querySelector('.customizable-combobox-datalist').showPopover();
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
  "source_name": "html/semantics/forms/customizable-combobox/customizable-combobox-appearance-ref.html"
}
```
