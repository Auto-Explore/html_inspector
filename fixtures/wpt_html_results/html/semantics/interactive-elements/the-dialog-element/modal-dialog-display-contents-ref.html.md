# html/semantics/interactive-elements/the-dialog-element/modal-dialog-display-contents-ref.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/modal-dialog-display-contents-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<title>Reference: Test that display: contents; on modal dialog & ::backdrop acts like display: block</title>
<meta charset="utf-8">
<link rel="author" title="Tim Nguyen" href="https://github.com/nt1m">
<p>Test passes if there is a green dialog</p>
<p>Dialog is display:block</p>
<p>Dialog::backdrop is display:block</p>
<dialog>Dialog Contents</dialog>
<style>
dialog {
    background-color: green;
}
</style>
<script>
document.querySelector("dialog").showModal();
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
        "byte_end": 383,
        "byte_start": 376,
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/modal-dialog-display-contents-ref.html"
}
```
