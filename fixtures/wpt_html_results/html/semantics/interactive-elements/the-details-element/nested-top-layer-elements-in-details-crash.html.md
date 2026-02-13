# html/semantics/interactive-elements/the-details-element/nested-top-layer-elements-in-details-crash.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-details-element/nested-top-layer-elements-in-details-crash.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype HTML>
<link rel=author href="mailto:vmpstr@chromium.org">
<link rel=help href="https://crbug.com/1273395">

<dialog id="parentElement">
  <details id="childElement" open="true" ontoggle="toggleHandler()">
    <dialog id="grandchildElement">
    </dialog>
  </details>
</dialog>
<script>
function toggleHandler() {
 grandchildElement.showModal();
 parentElement.showModal();
 childElement.open = false;
}
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.details.missing_summary",
      "message": "Element “details” is missing a required instance of child element “summary”.",
      "severity": "Warning",
      "span": {
        "byte_end": 277,
        "byte_start": 267,
        "col": 3,
        "line": 9
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
  "source_name": "html/semantics/interactive-elements/the-details-element/nested-top-layer-elements-in-details-crash.html"
}
```
