# html/semantics/forms/the-form-element/form-controls-nested-id-crash.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-form-element/form-controls-nested-id-crash.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<link rel="help" href="https://bugzilla.mozilla.org/show_bug.cgi?id=1949893">
<script>
document.addEventListener("DOMContentLoaded", () => {
  b.outerHTML = a.outerHTML
  a.replaceWith()
})
</script>
<fieldset form="a"></fieldset>
<form id="a">
<object>
<param id="b">
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.object.data.missing",
      "message": "Element “object” is missing required attribute “data”.",
      "severity": "Warning",
      "span": {
        "byte_end": 292,
        "byte_start": 284,
        "col": 1,
        "line": 12
      }
    },
    {
      "category": "Html",
      "code": "html.element.param.obsolete",
      "message": "The “param” element is obsolete. Use the “data” attribute of the “object” element to set the URL of the external resource.",
      "severity": "Warning",
      "span": {
        "byte_end": 307,
        "byte_start": 293,
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
  "source_name": "html/semantics/forms/the-form-element/form-controls-nested-id-crash.html"
}
```
