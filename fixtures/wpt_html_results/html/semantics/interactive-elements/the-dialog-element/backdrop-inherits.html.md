# html/semantics/interactive-elements/the-dialog-element/backdrop-inherits.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/backdrop-inherits.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel="match" href="backdrop-inherits-ref.html">
<link rel="help" href="https://drafts.csswg.org/css-position-4/#backdrop">
<style>
dialog {
  --backdrop-bg: green;
  visibility: hidden;
}

dialog::backdrop {
  position: absolute;
  top: 100px;
  left: 100px;
  height: 100px;
  width: 100px;
  visibility: visible;
  background-color: var(--backdrop-bg);
}
</style>
Test that ::backdrop inherits from its originating element. The test passes if
there is a green box and no red.
<dialog></dialog>
<script>
document.querySelector('dialog').showModal();
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/backdrop-inherits.html"
}
```
