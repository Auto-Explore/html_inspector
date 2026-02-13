# html/semantics/interactive-elements/the-dialog-element/top-layer-containing-block.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/top-layer-containing-block.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
<link rel="match" href="top-layer-containing-block-ref.html">
<link rel="help" href="https://fullscreen.spec.whatwg.org/#new-stacking-layer">
<style>
::backdrop {
    display: none;
}
</style>
</head>
<body>
<p>
This tests that a modal dialog's containing block is in the initial containing block and that it is unaffected by
ancestor elements with overflow or opacity.
<div style="position: absolute; top: 400px; opacity: 0.3">
    <dialog id="opaqueDialog" style="position: absolute; top: 250px; left: 0px; background-color: magenta; outline: none">
        This dialog should be unaffected by its ancestor with opacity.
    </dialog>
</div>
<div style="position: absolute; overflow: hidden; width: 500px; height: 150px; top: 400px; left: 300px">
    <dialog id="unclippedDialog" style="position: absolute; top: 200px; left: 0px; height: 100px; background-color: cyan">
        This dialog should be unaffected by its ancestor with overflow. It should not be clipped.
    </dialog>
</div>
<div style="position: absolute; top: 1000px; left: 1000px; width: 20px;">
    <dialog id="bottomDialog" style="position: absolute; top: 100px; height: 250px; width: 90%; background-color: yellow">
        This dialog should be onscreen with a width of 90% of the page. It is the child of an narrow element
        positioned off screen, but the containing block of a top layer element is the initial containing block, so its
        position and percent lengths are relative to that.
    </dialog>
</div>
<script>
document.getElementById('bottomDialog').showModal();
document.getElementById('unclippedDialog').showModal();
document.getElementById('opaqueDialog').showModal();
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/top-layer-containing-block.html"
}
```
