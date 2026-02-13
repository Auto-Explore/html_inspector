# html/semantics/forms/the-select-element/select-marker-end-aligned.tentative.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/select-marker-end-aligned.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<!-- Tests that <select> marker is end-aligned -->
<link rel=author href="mailto:pkotwicz@chromium.org">
<link rel="match" href="select-marker-end-aligned-ref.tentative.html">

<div>
  <select>
    <option></option>
  </select>
</div>

<style>
/* This is the only difference between the test and the reference: */
select {
  width:200px;
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
        "byte_end": 231,
        "byte_start": 222,
        "col": 13,
        "line": 8
      }
    },
    {
      "category": "Html",
      "code": "html.style.not_allowed_here",
      "message": "Element “style” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 259,
        "byte_start": 252,
        "col": 1,
        "line": 12
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
  "source_name": "html/semantics/forms/the-select-element/select-marker-end-aligned.tentative.html"
}
```
