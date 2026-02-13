# html/semantics/interactive-elements/the-dialog-element/top-layer-stacking-dynamic.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/top-layer-stacking-dynamic.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
<link rel="match" href="top-layer-stacking-dynamic-ref.html">
<link rel="help" href="https://fullscreen.spec.whatwg.org/#new-stacking-layer">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#the-dialog-element">
<style>
dialog {
    height: 150px;
    width: 150px;
    outline: none;
}

::backdrop {
    display: none;
}

.red {
    background-color: red;
    top: 200px;
}

#bottomDialog {
    background-color: blue;
    top: 50px;
}

#topDialog {
    background-color: green;
    top: 100px;
    left: 50px;
}
</style>
</head>
<body>
This tests top layer element stacking order after dynamically calling show/close and removal from the DOM tree.
The test passes if you see a green rectangle stacked on top of a blue rectangle, and see no red rectangles.

<dialog id="topDialog"></dialog>
<dialog id="bottomDialog"></dialog>
<dialog id="removedDialog" class="red">
    <dialog id="removedDialogChild" class="red"></dialog>
</dialog>
<script>
document.getElementById('topDialog').showModal();
var removedDialog = document.getElementById('removedDialog');
removedDialog.showModal();
document.getElementById('bottomDialog').showModal();
document.getElementById('removedDialogChild').showModal();
removedDialog.parentNode.removeChild(removedDialog);
document.getElementById('topDialog').close();
document.getElementById('topDialog').showModal();
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/top-layer-stacking-dynamic.html"
}
```
