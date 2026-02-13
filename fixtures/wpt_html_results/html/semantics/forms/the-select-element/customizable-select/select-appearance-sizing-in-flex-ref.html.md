# html/semantics/forms/the-select-element/customizable-select/select-appearance-sizing-in-flex-ref.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/customizable-select/select-appearance-sizing-in-flex-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<title>Test of sizing customizable select inside of flex</title>
<link rel=author href="mailto:dbaron@chromium.org">

<style>
  .container > * {
    display: inline-block;
    vertical-align: top;
    white-space: nowrap;
  }

  select {
    appearance: base-select;
  }

  ::picker-icon {
    display: none;
  }
</style>

<div class="container">
  <select>
    <option>First Option</option>
    <option>Second Option</option>
  </select
  ><div class="after">The content after the select.</div>
</div>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/forms/the-select-element/customizable-select/select-appearance-sizing-in-flex-ref.html"
}
```
