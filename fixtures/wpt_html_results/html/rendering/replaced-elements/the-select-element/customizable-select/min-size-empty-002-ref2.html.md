# html/rendering/replaced-elements/the-select-element/customizable-select/min-size-empty-002-ref2.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/replaced-elements/the-select-element/customizable-select/min-size-empty-002-ref2.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<link rel="mismatch" href="min-size-empty-002-notref.html">
<style>

select, select::picker(select) {
  appearance: base-select;
}
select::picker-icon {
  display: none;
}

select {
  width: 23px;
  height: calc(max(1lh, 24px) - 1px);
}

</style>

<select><option></option></select>
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
        "byte_end": 289,
        "byte_start": 280,
        "col": 17,
        "line": 19
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
  "source_name": "html/rendering/replaced-elements/the-select-element/customizable-select/min-size-empty-002-ref2.html"
}
```
