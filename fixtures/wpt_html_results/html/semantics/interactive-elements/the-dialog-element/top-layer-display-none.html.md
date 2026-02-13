# html/semantics/interactive-elements/the-dialog-element/top-layer-display-none.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/top-layer-display-none.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
<link rel="match" href="top-layer-display-none-ref.html">
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

.red {
    background-color: red;
    top: 200px;
}

#bottomDialog {
    background-color: blue;
    top: 50px;
    display: none;
}

#topDialog {
    background-color: green;
    top: 100px;
    left: 50px;
}
</style>
</head>
<body>
This tests that a top layer element is not rendered if it, or an ancestor, has display: none.
It passes if you see a green rectangle stacked on top of a blue rectangle, and see no red rectangles.

<dialog id="hiddenDialog" class="red" style="display: none;"></dialog>
<div id="container">
    <div>
        <dialog id="displayNoneChild1" class="red"></dialog>
        <dialog id="displayNoneChild2" class="red"></dialog>
    </div>
</div>
<dialog id="bottomDialog"></dialog>
<dialog id="topDialog"></dialog>
<script>
document.getElementById('hiddenDialog').showModal();
document.getElementById('displayNoneChild1').showModal();
document.getElementById('container').style.display = 'none';
document.getElementById('displayNoneChild2').showModal();

// Test that stacking works even if an element is added to the top layer when it has no renderer.
document.getElementById('bottomDialog').showModal();
document.getElementById('topDialog').showModal();
document.getElementById('bottomDialog').style.display = 'block';
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/top-layer-display-none.html"
}
```
