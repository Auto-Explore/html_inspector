# html/semantics/forms/the-select-element/customizable-select-listbox/customizable-select-listbox-appearance.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/customizable-select-listbox/customizable-select-listbox-appearance.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=help href="https://issues.chromium.org/issues/357649033">
<link rel=match href="customizable-select-listbox-appearance-ref.html">

<style>
select {
  appearance: base-select;
}
</style>

<select multiple>
  <option selected>select multiple</option>
  <option>two</option>
  <option>three</option>
</select>

<select size=4>
  <option selected>select size=4</option>
  <option>two</option>
  <option>three</option>
</select>

<select size=4 disabled>
  <option selected>select disabled</option>
  <option>two</option>
  <option>three</option>
  <option disabled>disabled</option>
</select>

<select size=4>
  <option selected>with disabled option</option>
  <option>two</option>
  <option>three</option>
  <option disabled>disabled</option>
</select>

<select size=4>
  <optgroup>
    <legend>optgroup</legend>
    <option>option in optgroup</option>
  </optgroup>
  <option>option out of optgroup</option>
</select>

<select size=4>
  <optgroup disabled>
    <legend>disabled optgroup</legend>
    <option>option in optgroup</option>
  </optgroup>
  <option>option out of optgroup</option>
</select>
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
  "source_name": "html/semantics/forms/the-select-element/customizable-select-listbox/customizable-select-listbox-appearance.html"
}
```
