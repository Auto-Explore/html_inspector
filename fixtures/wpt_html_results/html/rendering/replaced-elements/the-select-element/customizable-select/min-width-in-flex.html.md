# html/rendering/replaced-elements/the-select-element/customizable-select/min-width-in-flex.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/replaced-elements/the-select-element/customizable-select/min-width-in-flex.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<link rel="match" href="min-width-in-flex-ref.html">
<style>

select, select::picker(select) {
  appearance: base-select;
}

select {
  white-space: nowrap;
}

</style>

<div style="display:flex; flex-direction: row; width: 400px">
  <select style="align-self: flex-start">
    <option>This is an option</option>
  </select>
  <div style="color: transparent">
    This div has a lot of text in it but it should not make the select become smaller.
    A lot of text.  Really a lot of text.
  </div>
</div>
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
  "source_name": "html/rendering/replaced-elements/the-select-element/customizable-select/min-width-in-flex.html"
}
```
