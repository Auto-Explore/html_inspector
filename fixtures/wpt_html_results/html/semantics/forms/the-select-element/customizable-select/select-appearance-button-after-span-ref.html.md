# html/semantics/forms/the-select-element/customizable-select/select-appearance-button-after-span-ref.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/customizable-select/select-appearance-button-after-span-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=stylesheet href="resources/customizable-select-styles.css">

<div class=customizable-select-button>
  <span class=customizable-select-selectedcontent>option 1</span>
</div>
<div class=customizable-select-popover>
  Hello <button>Hello option 1</button>
  <div class="customizable-select-option selected">option 1</div>
  <div class=customizable-select-option>option 2</div>
</div>

<style>
  .customizable-select-button {
    anchor-name: --select;
  }
  .customizable-select-popover {
    position-anchor: --select;
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
        "byte_end": 415,
        "byte_start": 408,
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
  "source_name": "html/semantics/forms/the-select-element/customizable-select/select-appearance-button-after-span-ref.html"
}
```
