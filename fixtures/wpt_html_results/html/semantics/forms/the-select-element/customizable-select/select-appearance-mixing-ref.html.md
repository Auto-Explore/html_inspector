# html/semantics/forms/the-select-element/customizable-select/select-appearance-mixing-ref.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/customizable-select/select-appearance-mixing-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<style>
.custom, .custom::picker(select) {
  appearance: base-select;
}
</style>

<select multiple>
  <option>normal</option>
  <div>
    <option>
      <div>div</div>
      <span>span</span>
      text
    </option>
  </div>
</select>

<select multiple class=custom>
  <option>normal</option>
  <option>base-select</option>
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
  "source_name": "html/semantics/forms/the-select-element/customizable-select/select-appearance-mixing-ref.html"
}
```
