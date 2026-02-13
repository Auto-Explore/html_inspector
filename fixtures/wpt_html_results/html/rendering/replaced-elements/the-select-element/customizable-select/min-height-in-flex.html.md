# html/rendering/replaced-elements/the-select-element/customizable-select/min-height-in-flex.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/replaced-elements/the-select-element/customizable-select/min-height-in-flex.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<link rel="match" href="min-height-in-flex-ref.html">
<style>

select, select::picker(select) {
  appearance: base-select;
}
select::picker-icon {
  font-size: 60px;
}

</style>

<div style="display:flex; flex-direction: column; align-items: flex-start; height: 100px;">
  <select>
    <option>Option</option>
  </select>
  <div style="color: transparent">
    This div has a lot of text in it but it should not make the select become smaller.
    A lot of text.  Really a lot of text.
    A lot of text.  Really a lot of text.
    A lot of text.  Really a lot of text.
    A lot of text.  Really a lot of text.
    A lot of text.  Really a lot of text.
    A lot of text.  Really a lot of text.
    A lot of text.  Really a lot of text.
    A lot of text.  Really a lot of text.
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
  "source_name": "html/rendering/replaced-elements/the-select-element/customizable-select/min-height-in-flex.html"
}
```
