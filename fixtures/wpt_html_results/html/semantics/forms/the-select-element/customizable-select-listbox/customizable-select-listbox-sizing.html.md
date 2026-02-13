# html/semantics/forms/the-select-element/customizable-select-listbox/customizable-select-listbox-sizing.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/customizable-select-listbox/customizable-select-listbox-sizing.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=help href="https://github.com/w3c/csswg-drafts/issues/12510#issuecomment-3361831625">
<link rel=match href="customizable-select-listbox-sizing-ref.html">

<style>
select {
  appearance: base-select;
}
#big {
  font-size: 26px;
}
</style>

<select multiple>
  <option selected>one</option>
  <option>two</option>
  <option>three</option>
  <option>four</option>
</select>

<select size=2>
  <option selected>one</option>
  <option>two</option>
  <option>three</option>
  <option>four</option>
</select>

<select size=2 id=big>
  <option selected>big one</option>
  <option>big two</option>
  <option>big three</option>
  <option>big four</option>
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
  "source_name": "html/semantics/forms/the-select-element/customizable-select-listbox/customizable-select-listbox-sizing.html"
}
```
