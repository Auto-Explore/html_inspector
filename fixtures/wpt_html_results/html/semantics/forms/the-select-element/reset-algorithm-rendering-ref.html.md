# html/semantics/forms/the-select-element/reset-algorithm-rendering-ref.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/reset-algorithm-rendering-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<body>

<form>
<select>
<option>Default</option>
<option>Another</option>
</select>

<select>
<option>Another</option>
<option selected>Default</option>
</select>

<select multiple>
<option>option 1</option>
<option>option 2</option>
</select>
</form>

</body>
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
  "source_name": "html/semantics/forms/the-select-element/reset-algorithm-rendering-ref.html"
}
```
