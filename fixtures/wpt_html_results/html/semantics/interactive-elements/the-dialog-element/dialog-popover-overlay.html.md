# html/semantics/interactive-elements/the-dialog-element/dialog-popover-overlay.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/dialog-popover-overlay.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<link rel=author href="mailto:masonf@chromium.org">
<link rel=help href="https://issues.chromium.org/issues/388538944">
<link rel=match href="dialog-popover-overlay-ref.html">

<dialog open popover></dialog>
<div class=blocker></div>

<style>
* {
  margin: 0;
  padding: 0;
}
dialog,div {
  left: 0;
  top: 0;
  border: 0;
  width: 300px;
  height: 300px;
  background: green;
}
.blocker {
  contain: layout;
  background: red;
}
</style>

<script>
  const dialog = document.querySelector('dialog');
  dialog.showPopover();
  dialog.blur();
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.style.not_allowed_here",
      "message": "Element “style” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 281,
        "byte_start": 274,
        "col": 1,
        "line": 10
      }
    },
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/dialog-popover-overlay.html"
}
```
