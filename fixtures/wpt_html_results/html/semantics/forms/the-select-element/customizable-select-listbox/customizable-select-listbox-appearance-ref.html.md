# html/semantics/forms/the-select-element/customizable-select-listbox/customizable-select-listbox-appearance-ref.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/customizable-select-listbox/customizable-select-listbox-appearance-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=stylesheet href="resources/customizable-select-listbox.css">
<link rel=stylesheet href="../customizable-select/resources/customizable-select-styles.css">

<div class=customizable-select-listbox size=4>
  <div class="customizable-select-option selected">select multiple</div>
  <div class=customizable-select-option>two</div>
  <div class=customizable-select-option>three</div>
</div>

<div class=customizable-select-listbox size=4>
  <div class="customizable-select-option selected">select size=4</div>
  <div class=customizable-select-option>two</div>
  <div class=customizable-select-option>three</div>
</div>

<div class="customizable-select-listbox disabled" size=4>
  <div class="customizable-select-option selected">select disabled</div>
  <div class=customizable-select-option>two</div>
  <div class=customizable-select-option>three</div>
  <div class=customizable-select-option>disabled</div>
</div>

<div class=customizable-select-listbox size=4>
  <div class="customizable-select-option selected">with disabled option</div>
  <div class=customizable-select-option>two</div>
  <div class=customizable-select-option>three</div>
  <div class="customizable-select-option disabled">disabled</div>
</div>

<div class=customizable-select-listbox size=4>
  <div class=customizable-select-legend>optgroup</div>
  <div class=customizable-select-option>option in optgroup</div>
  <div class=customizable-select-option>option out of optgroup</div>
</div>

<div class=customizable-select-listbox size=4>
  <div class="customizable-select-legend disabled">disabled optgroup</div>
  <div class="customizable-select-option disabled">option in optgroup</div>
  <div class=customizable-select-option>option out of optgroup</div>
</div>
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
  "source_name": "html/semantics/forms/the-select-element/customizable-select-listbox/customizable-select-listbox-appearance-ref.html"
}
```
