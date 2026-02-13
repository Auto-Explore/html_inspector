# html/semantics/forms/the-select-element/customizable-select/select-appearance-mixing.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/customizable-select/select-appearance-mixing.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=help href="https://issues.chromium.org/issues/393500003">
<link rel=help href="https://chromium-review.googlesource.com/c/chromium/src/+/7032708/comment/aa4b509b_fec51624/">
<link rel=match href="select-appearance-mixing-ref.html">

<style>
.custom, .custom::picker(select) {
  appearance: base-select;
}
</style>

<select multiple>
  <option>normal</option>
  <div style="appearance:base-select">
    <option style="appearance:base-select">
      <div>div</div>
      <span>span</span>
      text
    </option>
  </div>
</select>

<select multiple class=custom>
  <option>normal</option>
  <option style="appearance:base-select">base-select</option>
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
  "source_name": "html/semantics/forms/the-select-element/customizable-select/select-appearance-mixing.html"
}
```
