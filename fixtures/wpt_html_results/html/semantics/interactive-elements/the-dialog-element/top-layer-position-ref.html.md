# html/semantics/interactive-elements/the-dialog-element/top-layer-position-ref.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/top-layer-position-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>

<style>
dialog {
  background-color: green;
  height: 50px;
  width: 50px;
  border: none;
  padding: 0;
  margin: 0;

  position: absolute;
  top: 100px;
  left: 100px;
}
</style>

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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/top-layer-position-ref.html"
}
```
