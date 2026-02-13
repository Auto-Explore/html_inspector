# html/semantics/interactive-elements/the-dialog-element/fixed-position-child-with-transformed-ancestor.tentative.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/fixed-position-child-with-transformed-ancestor.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<title>Test that a fixed positioned child of a dialog is aligned to the viewport</title>
<head>
<link rel="match" href="fixed-position-child-with-fixed-position-cb-ref.html">
<link rel="help" href="https://fullscreen.spec.whatwg.org/#new-stacking-layer">
<style>
::backdrop {
    display: none;
}
#dialog {
    outline: none;
}
</style>
</head>
<body>
<div style="scale: 1">
    <dialog id="dialog">
        Dialog should be centered.
        <div style="position: fixed; top: 0px; left: 0px">This fixed positioned element is aligned to viewport top-left.</div>
    </dialog>
</div>
<script>
dialog.showModal();
</script>
</body>
</html>
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/fixed-position-child-with-transformed-ancestor.tentative.html"
}
```
