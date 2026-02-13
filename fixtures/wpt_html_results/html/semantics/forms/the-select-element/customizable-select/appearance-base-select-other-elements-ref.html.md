# html/semantics/forms/the-select-element/customizable-select/appearance-base-select-other-elements-ref.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/customizable-select/appearance-base-select-other-elements-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<h2>appearance:base-select should not affect the rendering of non-select elements.</h2>

<button>button</button>
<input type=checkbox>
<input type=checkbox checked>
<input type=radio>
<input type=radio checked>
<input type=range>
<input type=color>
<input type=date>
<input type=datetime-local>
<input type=time>
<input type=week>
<input type=month>
<input type=file>
<input type=search>
<input>
<input type=password>
<input type=number>
<textarea></textarea>
<progress max=100 value=50></progress>
<meter min=0 max=100 low=33 high=66 optimum=80 value=50></meter>
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
  "source_name": "html/semantics/forms/the-select-element/customizable-select/appearance-base-select-other-elements-ref.html"
}
```
