# html/semantics/interactive-elements/the-dialog-element/modal-dialog-display-contents.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/modal-dialog-display-contents.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<title>Test that display: contents; on modal dialog & ::backdrop acts like display: block</title>
<meta charset="utf-8">
<link rel="author" title="Tim Nguyen" href="https://github.com/nt1m">
<link rel="help" href="https://fullscreen.spec.whatwg.org/#new-stacking-layer">
<link rel="help" href="https://html.spec.whatwg.org/multipage/interactive-elements.html#the-dialog-element">
<link rel="match" href="modal-dialog-display-contents-ref.html">
<p>Test passes if there is a green dialog</p>
<p>Dialog is display:<span id="computed-value"></span></p>
<p>Dialog::backdrop is display:<span id="computed-value-backdrop"></span></p>
<dialog>Dialog Contents</dialog>
<style>
dialog {
    display: contents;
    background-color: green;
}
dialog::backdrop {
    display: contents;
}
</style>
<script>
dialog = document.querySelector("dialog");
dialog.showModal();
document.getElementById("computed-value").textContent = getComputedStyle(dialog).display;
document.getElementById("computed-value-backdrop").textContent = getComputedStyle(dialog, "::backdrop").display;
</script>
</html>
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
        "byte_end": 691,
        "byte_start": 684,
        "col": 1,
        "line": 13
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/interactive-elements/the-dialog-element/modal-dialog-display-contents.html"
}
```
