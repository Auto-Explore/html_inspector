# html/semantics/interactive-elements/the-dialog-element/modal-dialog-in-table-column.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/modal-dialog-in-table-column.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
<title>Modal dialog inside display: table-column should not generate box</title>
<link rel="match" href="modal-dialog-in-table-column-ref.html">
<link rel="help" href="https://fullscreen.spec.whatwg.org/#new-stacking-layer">
<style>
dialog {
    background: green;
    border-color: green;
}
div {
    display: table-column;
}
</style>
</head>
<body>
<p>The test passes if you see no green rectangle.
<div>
<dialog id="dialog"></dialog>
</div>
<script>
document.getElementById('dialog').showModal();
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/modal-dialog-in-table-column.html"
}
```
