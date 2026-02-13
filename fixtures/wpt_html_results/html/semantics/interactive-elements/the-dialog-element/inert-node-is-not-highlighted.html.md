# html/semantics/interactive-elements/the-dialog-element/inert-node-is-not-highlighted.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/inert-node-is-not-highlighted.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
<link rel="match" href="inert-node-is-not-highlighted-ref.html">
<link rel="help" href="https://html.spec.whatwg.org/multipage/interaction.html#inert-subtrees">
<style>
::backdrop {
    display: none;
}
</style>
</head>
<body>
<p>Test that inert nodes are not painted as being selected. The test passes if
none of the text outside the dialog is highlighted when selected.</p>

<p>Although not shown as selected, the inert nodes are in window.getSelection()
and copied to the clipboard, which is the same behavior as user-select:
none (crbug.com/147490).</p>

<br>  <!-- Needed to the trigger the bug. -->
This text shouldn't be highlighted as selected.

<dialog>
    <div id="selectable">I'm selectable.</div>
</dialog>

<script>
dialog = document.querySelector('dialog');
dialog.showModal();
document.execCommand('SelectAll');
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/inert-node-is-not-highlighted.html"
}
```
