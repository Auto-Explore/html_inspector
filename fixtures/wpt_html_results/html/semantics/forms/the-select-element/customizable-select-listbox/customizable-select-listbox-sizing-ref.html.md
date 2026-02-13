# html/semantics/forms/the-select-element/customizable-select-listbox/customizable-select-listbox-sizing-ref.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/customizable-select-listbox/customizable-select-listbox-sizing-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=stylesheet href="resources/customizable-select-listbox.css">
<link rel=stylesheet href="../customizable-select/resources/customizable-select-styles.css">

<style>
#size4 {
  height: calc(24px * 4);
}
#size2 {
  height: calc(24px * 2);
}
#big2 {
  font-size: 26px;
  height: 2lh;
}
</style>

<div class=customizable-select-listbox id=size4>
  <div class="customizable-select-option selected">one</div>
  <div class=customizable-select-option>two</div>
  <div class=customizable-select-option>three</div>
  <div class=customizable-select-option>four</div>
</div>

<div class=customizable-select-listbox id=size2>
  <div class="customizable-select-option selected">one</div>
  <div class=customizable-select-option>two</div>
  <div class=customizable-select-option>three</div>
  <div class=customizable-select-option>four</div>
</div>

<div class=customizable-select-listbox id=big2>
  <div class="customizable-select-option selected">big one</div>
  <div class=customizable-select-option>big two</div>
  <div class=customizable-select-option>big three</div>
  <div class=customizable-select-option>big four</div>
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
  "source_name": "html/semantics/forms/the-select-element/customizable-select-listbox/customizable-select-listbox-sizing-ref.html"
}
```
