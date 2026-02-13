# html/semantics/forms/the-select-element/select-marker-end-aligned-ref.tentative.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/select-marker-end-aligned-ref.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>

<div>
  <select>
    <option></option>
  </select>
</div>

<style>
/* This is the only difference between the test and the reference: */
select {
  width:100px;
}

div {
  width:300px;
  display:flex;
  justify-content:flex-end;
}
select {
  border-style:none;
  background-color:rgba(0,0,0,0)
}
select, ::picker(select) {
  appearance: base-select;
}
</style>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.option.empty_without_label",
      "message": "Element “option” without attribute “label” must not be empty.",
      "severity": "Warning",
      "span": {
        "byte_end": 55,
        "byte_start": 46,
        "col": 13,
        "line": 5
      }
    },
    {
      "category": "Html",
      "code": "html.style.not_allowed_here",
      "message": "Element “style” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 83,
        "byte_start": 76,
        "col": 1,
        "line": 9
      }
    },
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
  "source_name": "html/semantics/forms/the-select-element/select-marker-end-aligned-ref.tentative.html"
}
```
