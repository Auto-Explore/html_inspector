# html/semantics/interactive-elements/the-dialog-element/modal-dialog-generated-content.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/modal-dialog-generated-content.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel="match" href="modal-dialog-generated-content-ref.html">
<link rel="help" href="https://fullscreen.spec.whatwg.org/#new-stacking-layer">
<style>
dialog {
    padding: 0px;
    border: none;
    margin: 0px;
    top: 100px;
    left: 100px;
    height: 100px;
    width: 100px;
    background: green;
    outline: none;
}

dialog::before {
    content: '::before';
    position: absolute;
    top: 0px;
}

dialog::after {
    content: '::after';
    position: absolute;
    bottom: 0px;
}

dialog::backdrop {
    position: absolute;
    top: 100px;
    left: 300px;
    height: 100px;
    width: 100px;
    background: green;
    content: 'THIS TEXT SHOULD NOT BE SEEN';
}

dialog::backdrop::before {
    content: '::backdrop::before';
    position: absolute;
    top: 0px;
    background: red;
}
dialog::backdrop::after {
    content: '::backdrop::after';
    position: absolute;
    bottom: 0px;
    background: red;
}
</style>
<body>
Test for a modal dialog with ::before, ::after, and ::backdrop. The test passes
if there are two green boxes, one with the texts "::before" and "::after" in it.
<dialog></dialog>
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/modal-dialog-generated-content.html"
}
```
