# html/semantics/interactive-elements/the-dialog-element/removed-element-is-removed-from-top-layer.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/removed-element-is-removed-from-top-layer.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
<link rel="match" href="removed-element-is-removed-from-top-layer-ref.html">
<link rel="help" href="https://fullscreen.spec.whatwg.org/#new-stacking-layer">
<style>
dialog {
    height: 100px;
    width: 100px;
}

::backdrop {
    display: none;
}

#bottomDialog {
    background-color: blue;
    top: 231px;
}

#topDialog {
    background-color: green;
    top: 50px;
    left: 50px;
}
</style>
</head>
<body>
<p>Bug <a href="https://bugs.webkit.org/show_bug.cgi?id=105489">105489</a>: Elements must be reattached when inserted/removed from top layer
<p>The test passes if you see a green rectangle stacked on top of a blue rectangle.
<dialog id="bottomDialog"></dialog>
<dialog id="topDialog"></dialog>
<script>
document.getElementById('topDialog').showModal();
var bottomDialog = document.getElementById('bottomDialog');
bottomDialog.showModal();
bottomDialog.offsetTop;  // force a layout
var parent = bottomDialog.parentNode;
parent.removeChild(bottomDialog);
parent.appendChild(bottomDialog);
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/removed-element-is-removed-from-top-layer.html"
}
```
