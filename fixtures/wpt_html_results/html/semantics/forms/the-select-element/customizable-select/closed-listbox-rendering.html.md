# html/semantics/forms/the-select-element/customizable-select/closed-listbox-rendering.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/customizable-select/closed-listbox-rendering.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=help href="https://github.com/whatwg/html/issues/9799">
<link rel=match href="closed-listbox-rendering-ref.html">
<style>
select {
  appearance: base-select;
}
</style>
<select>
  <option>option</option>
</select>
<select>
  <optgroup label=label>
    <option>option</option>
  </optgroup>
</select>
<select>
  <option>one</option>
  <hr>
  <option>two</option>
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
  "source_name": "html/semantics/forms/the-select-element/customizable-select/closed-listbox-rendering.html"
}
```
