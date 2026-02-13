# html/semantics/interactive-elements/the-dialog-element/top-layer-remove-popover-attribute.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/top-layer-remove-popover-attribute.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<meta charset="utf-8">
<meta name="assert" content="Removing the popover attribute of a hidden popover should not remove the dialog from the top layer.">
<head>
  <title>Shown modal dialog where the popover attribute is removed</title>
  <link rel="help" href="https://html.spec.whatwg.org/multipage/popover.html#hide-popover-algorithm">
  <link rel="match" href="top-layer-remove-popover-attribute-ref.html">
</head>
<body>
  <dialog popover style="padding: 2em"></dialog>
  <script>
    const d = document.querySelector("dialog");
    d.showModal();
    d.popover = null;
  </script>
</body>
<html>
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/top-layer-remove-popover-attribute.html"
}
```
