# html/semantics/interactive-elements/the-dialog-element/modal-dialog-in-object.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/modal-dialog-in-object.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Modal dialog inside object should not generate box</title>
<link rel=match href="modal-dialog-in-object-ref.html">
<link rel=help href="https://github.com/w3c/csswg-drafts/issues/6939">
<link rel=help href="https://fullscreen.spec.whatwg.org/#new-stacking-layer">
<style>
dialog {
  background: green;
  border-color: green;
}
</style>
<object width="200" type="image/svg+xml" data="../../../../images/100px-green-rect.svg">
  <dialog id="dialog"></dialog>
</object>
<script>
document.getElementById('dialog').showModal();
</script>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/interactive-elements/the-dialog-element/modal-dialog-in-object.html"
}
```
