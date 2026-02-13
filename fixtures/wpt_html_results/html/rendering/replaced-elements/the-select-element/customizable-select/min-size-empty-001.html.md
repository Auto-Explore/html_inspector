# html/rendering/replaced-elements/the-select-element/customizable-select/min-size-empty-001.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/replaced-elements/the-select-element/customizable-select/min-size-empty-001.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<link rel="match" href="min-size-empty-001-ref.html">
<style>

select, select::picker(select) {
  appearance: base-select;
}
select::picker-icon {
  display: none;
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
  "source_name": "html/rendering/replaced-elements/the-select-element/customizable-select/min-size-empty-001.html"
}
```
