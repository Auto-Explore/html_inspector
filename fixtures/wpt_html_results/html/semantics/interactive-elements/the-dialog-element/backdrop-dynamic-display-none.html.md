# html/semantics/interactive-elements/the-dialog-element/backdrop-dynamic-display-none.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/backdrop-dynamic-display-none.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<title>Test that adding display: none; dynamically on ::backdrop makes it disappear</title>
<meta charset="utf-8">
<link rel="author" title="Tim Nguyen" href="https://github.com/nt1m">
<link rel="match" href="backdrop-dynamic-display-none-ref.html">
<link rel="help" href="https://fullscreen.spec.whatwg.org/#new-stacking-layer">
<p>Test passes if there is no red.</p>
<dialog></dialog>
<style>
dialog { visibility: hidden; }
::backdrop { background-color: red; }
.hidden-backdrop::backdrop {
    display: none;
}
</style>
<script>
dialog = document.querySelector("dialog");
dialog.showModal();
requestAnimationFrame(() => {
    dialog.classList.add("hidden-backdrop");
});
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
        "byte_end": 417,
        "byte_start": 410,
        "col": 1,
        "line": 10
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/backdrop-dynamic-display-none.html"
}
```
