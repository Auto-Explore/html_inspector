# html/semantics/forms/the-select-element/customizable-select/select-grid-before-after.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/customizable-select/select-grid-before-after.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=help href="https://issues.chromium.org/issues/452087739">
<link rel=match href="select-grid-before-after-ref.html">

<style>
  select {
    appearance: base-select;

    &::picker-icon {
      display: none;
    }
  }

  select {
    display: inline-grid;
    width: 200px;
    height: 200px;
    box-sizing: border-box;
    vertical-align: top;
    place-items: center;
    background: orange;
    padding: 8px;
    margin: 0;
    border: none;
    border-radius: 0;
    grid-template-rows: 1fr 1fr;
    grid-template-columns: 1fr 1fr;
    gap: 0;

    &::before {
      content: 'before';
      grid-row: 1;
      grid-column: 1;
      background: yellow;
      padding: 8px;
    }

    &::after {
      content: 'after';
      grid-row: 2;
      grid-column: 2;
      background: yellow;
      padding: 8px;
    }
  }
</style>

<select></select>
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
  "source_name": "html/semantics/forms/the-select-element/customizable-select/select-grid-before-after.html"
}
```
