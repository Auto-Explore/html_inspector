# html/semantics/forms/the-select-element/customizable-select/select-appearance-sizing-in-flex.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/customizable-select/select-appearance-sizing-in-flex.html",
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
<link rel=help href="https://github.com/whatwg/html/issues/9799">
<link rel=help href="https://github.com/w3c/csswg-drafts/issues/12085">
<link rel=match href="select-appearance-sizing-in-flex-ref.html">
<meta name="assert" content="The min-content size of a customizable select is a function of its content.">

<style>
  .container {
    display: inline-flex;
    width: 15em;
  }

  select {
    appearance: base-select;
  }

  select, .after {
    white-space: nowrap;
  }

  /* ::picker-icon will render inline in this test but on a new line in the
   * reference because the select's inner element is display:block and can't be
   * changed. */
  ::picker-icon {
    display: none;
  }
</style>

<div class="container">
  <select>
    <option>First Option</option>
    <option>Second Option</option>
  </select>
  <div class="after">The content after the select.</div>
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
  "source_name": "html/semantics/forms/the-select-element/customizable-select/select-appearance-sizing-in-flex.html"
}
```
