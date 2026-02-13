# html/semantics/forms/the-select-element/customizable-select/selectedcontent-color.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/customizable-select/selectedcontent-color.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta name=fuzzy content="maxDifference=0-41;totalPixels=0-1">
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=match href="selectedcontent-color-ref.html">

<select>
  <button>
    <selectedcontent style="color:blue"></selectedcontent>
  </button>
  <option>option</option>
</select>

<style>
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
      "code": "html.style.not_allowed_here",
      "message": "Element “style” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 322,
        "byte_start": 315,
        "col": 1,
        "line": 13
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
  "source_name": "html/semantics/forms/the-select-element/customizable-select/selectedcontent-color.html"
}
```
