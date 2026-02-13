# html/semantics/interactive-elements/the-dialog-element/dialog-insert-disconnected-node-crash.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/dialog-insert-disconnected-node-crash.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>The container node of the dialog element should be connected to a document.</title>
<link rel="author" title="Peng Zhou" href="mailto:zhoupeng.1996@bytedance.com">

<!-- This test passes if it does not crash. -->

<div id="target"></div>
<dialog open>Dialog</dialog>

<script>
  const dialog = document.querySelector('dialog');
  const node = document.createElement('div');
  node.appendChild(dialog);
  document.getElementById('target').appendChild(node);
</script>
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/dialog-insert-disconnected-node-crash.html"
}
```
