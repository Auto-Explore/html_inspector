# html/semantics/interactive-elements/the-dialog-element/top-layer-nesting.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/top-layer-nesting.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
<link rel="match" href="top-layer-nesting-ref.html">
<link rel="help" href="https://fullscreen.spec.whatwg.org/#new-stacking-layer">
<style>
dialog {
    height: 150px;
    width: 150px;
    outline: none;
}

::backdrop {
    display: none;
}

#bottomDialog {
    background-color: yellow;
    top: 100px;
    z-index: 1000;
}

#middleDialog {
    background-color: blue;
    top: 150px;
    left: 50px;
    z-index: -500;
}

#topDialog {
    background-color: green;
    top: 200px;
    left: 100px;
    z-index: -1000;
}

.red {
    background-color: red;
    top: 250px;
    left: 0px;
}
</style>
</head>
<body>
This tests that top layer elements are stacked correctly even if nested in the DOM tree.
The test passes if you see no red rectangles and see 3 rectangles stacked in the following order (from bottom to top): yellow, blue, green.

<dialog id="topDialog">
    <dialog id="middleDialog">
        <dialog id="bottomDialog">
            <dialog id="hiddenDialog" class="red">
                <dialog id="hiddenDialogChild" class="red"></dialog>
            </dialog>
        </dialog>
    </dialog>
</dialog>
<script>
document.getElementById('hiddenDialogChild').showModal();
document.getElementById('hiddenDialog').showModal();
document.getElementById('bottomDialog').showModal();
document.getElementById('middleDialog').showModal();
document.getElementById('topDialog').showModal();
document.getElementById('hiddenDialog').close();
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/top-layer-nesting.html"
}
```
