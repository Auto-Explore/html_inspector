# html/semantics/interactive-elements/the-dialog-element/modal-dialog-sibling-ref.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/modal-dialog-sibling-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
<style>
dialog {
    background: green;
    border-color: green;
}
</style>
</head>
<body>
<p>Bug <a href="http://webkit.org/b/103477">103477</a>: Make
NodeRenderingContext::parentRenderer and nextRenderer top layer aware
<p>The test passes if you see a green rectangle in the center of the viewport.
<dialog id="dialog"></dialog>
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/modal-dialog-sibling-ref.html"
}
```
