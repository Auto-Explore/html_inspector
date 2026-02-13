# html/rendering/non-replaced-elements/flow-content-0/support/dialog-framed.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/flow-content-0/support/dialog-framed.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<style>
 html { color: red }
</style>
<dialog id=dialog-closed></dialog>
<dialog id=dialog-open open></dialog>
<dialog id=dialog-modal></dialog>
<script>
window.dialogClosed = document.getElementById('dialog-closed');
window.dialogOpen = document.getElementById('dialog-open');
window.dialogModal = document.getElementById('dialog-modal');
dialogModal.showModal();
</script>
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
  "source_name": "html/rendering/non-replaced-elements/flow-content-0/support/dialog-framed.html"
}
```
