# html/semantics/interactive-elements/the-dialog-element/modal-dialog-backdrop-opacity.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/modal-dialog-backdrop-opacity.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel="author" title="Tim Nguyen" href="https://github.com/nt1m">
<link rel="match" href="modal-dialog-backdrop-opacity-ref.html">
<link rel="help" href="https://fullscreen.spec.whatwg.org/#user-agent-level-style-sheet-defaults">
<style>
dialog::backdrop {
    background-color: rgb(0, 128, 0);
    opacity: 0.5;
}
dialog:focus {
  outline: none;
}
</style>
<body>
<dialog>Test passes if you see a green backdrop at half opacity.</dialog>
<script>
document.querySelector('dialog').showModal();
</script>
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/modal-dialog-backdrop-opacity.html"
}
```
