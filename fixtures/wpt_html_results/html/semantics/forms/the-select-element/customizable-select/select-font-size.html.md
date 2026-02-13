# html/semantics/forms/the-select-element/customizable-select/select-font-size.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/customizable-select/select-font-size.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<!-- Tests that select respects explicit size -->
<meta name=fuzzy content="maxDifference=0-55;totalPixels=0-32">
<link rel=author href="mailto:pkotwicz@chromium.org">
<link rel="match" href="select-font-size-ref.html">

<style>
select {
  font-size:48px;
}
</style>
<select>
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
        "byte_end": 336,
        "byte_start": 329,
        "col": 1,
        "line": 16
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
  "source_name": "html/semantics/forms/the-select-element/customizable-select/select-font-size.html"
}
```
