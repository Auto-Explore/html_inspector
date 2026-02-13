# html/semantics/interactive-elements/the-dialog-element/top-layer-backdrop-remove-add-ordering.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/top-layer-backdrop-remove-add-ordering.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=help href="https://issues.chromium.org/issues/360158414">
<link rel=match href="top-layer-backdrop-remove-add-ordering-ref.html">

<style>
#popover1 {
  top: 100px;
  left: 100px;
  width: 100px;
  height: 100px;
}
</style>

<dialog id=dialog1>Dialog1</dialog>
<div popover=manual id=popover1>Popover1</div>
<script>
const popover1 = document.getElementById('popover1');
const dialog1 = document.getElementById('dialog1');

dialog1.showModal();
popover1.showPopover();

dialog1.close();
dialog1.showModal();
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/top-layer-backdrop-remove-add-ordering.html"
}
```
