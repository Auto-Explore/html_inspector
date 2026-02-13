# html/semantics/forms/the-select-element/customizable-select/resources/selectedcontent-restore-iframe.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/customizable-select/resources/selectedcontent-restore-iframe.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<style>
select,::picker(select) {
  appearance: base-select;
}
</style>
<form action="blank.html">
  <select>
    <button>
      <selectedcontent></selectedcontent>
    </button>
    <option value=one>one<span>span</span></option>
    <option value=two>two<span>span</span></option>
  </select>
</form>
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
  "source_name": "html/semantics/forms/the-select-element/customizable-select/resources/selectedcontent-restore-iframe.html"
}
```
