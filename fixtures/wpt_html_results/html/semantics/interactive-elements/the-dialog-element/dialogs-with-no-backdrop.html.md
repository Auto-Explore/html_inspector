# html/semantics/interactive-elements/the-dialog-element/dialogs-with-no-backdrop.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/dialogs-with-no-backdrop.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel="match" href="dialogs-with-no-backdrop-ref.html">
<link rel="help" href="https://fullscreen.spec.whatwg.org/#new-stacking-layer">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#the-dialog-element">
<style>
dialog::backdrop {
    position: absolute;
    top: 100px;
    left: 100px;
    height: 100px;
    width: 100px;
    background: red;
}

#display-none-backdrop::backdrop {
    display: none;
}
</style>
<body>
Test that ::backdrop is not shown for non-open or non-modal dialogs.
The test passes if there is no red shown.
<dialog id="never-opened-dialog"></dialog>
<dialog id="display-none-dialog" style="display: none"></dialog>
<dialog id="non-modal-dialog" style="visibility: hidden"></dialog>
<dialog id="display-none-backdrop" style="visibility: hidden"></dialog>
<dialog id="closed-dialog"></dialog>
<dialog id="removed-dialog"></dialog>
<script>
document.getElementById('display-none-dialog').showModal();
document.getElementById('non-modal-dialog').show();
document.getElementById('display-none-backdrop').showModal();

var closedDialog = document.getElementById('closed-dialog');
closedDialog.showModal();
closedDialog.close();

var removedDialog = document.getElementById('removed-dialog');
removedDialog.showModal();
removedDialog.parentNode.removeChild(removedDialog);
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/dialogs-with-no-backdrop.html"
}
```
