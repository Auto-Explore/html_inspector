# html/semantics/interactive-elements/the-dialog-element/top-layer-remove-popover-attribute-ref.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/top-layer-remove-popover-attribute-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<meta charset="utf-8">
<head>
  <title>Shown modal dialog where the popover attribute is removed</title>
</head>
<body>
  <dialog popover style="padding: 2em"></dialog>
  <script>
    const d = document.querySelector("dialog");
    d.showModal();
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/top-layer-remove-popover-attribute-ref.html"
}
```
