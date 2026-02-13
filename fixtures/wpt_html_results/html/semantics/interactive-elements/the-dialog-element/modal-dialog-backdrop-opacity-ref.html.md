# html/semantics/interactive-elements/the-dialog-element/modal-dialog-backdrop-opacity-ref.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/modal-dialog-backdrop-opacity-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<meta charset="utf-8">
<link rel="stylesheet" href="resources/dialog.css">
<style>
.backdrop {
    display: block;
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;

    background-color: green;
    opacity: 0.5;
}
</style>
<body>
<div class="backdrop"></div>
<div class="pseudodialog">Test passes if you see a green backdrop at half opacity.</div>
</body>
</html>
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/modal-dialog-backdrop-opacity-ref.html"
}
```
