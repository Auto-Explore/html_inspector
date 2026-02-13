# html/semantics/interactive-elements/the-dialog-element/backdrop-descendant-selector.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/backdrop-descendant-selector.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
<link rel="match" href="backdrop-descendant-selector-ref.html">
<link rel="help" href="https://fullscreen.spec.whatwg.org/#new-stacking-layer">
<style>
dialog {
    visibility: hidden;
}

::backdrop {
    visibility: visible;
    position: absolute;
    height: 100px;
    width: 100px;
    background: red;
}

/* This shouldn't be matched, dialog is not the parent of ::backdrop.
 * It is given high specificity so we actually test something.
 */
#dialog-parent > #dialog > ::backdrop,
#dialog-parent > #dialog ::backdrop {
    background: red;
}

#dialog-parent > ::backdrop {
    top: 100px;
    left: 100px;
    background: green;
}

#backdrop-ancestor ::backdrop {
    top: 100px;
    left: 300px;
    background: green;
}
</style>
</head>
<body>
Test ::backdrop used in descendant selectors. The test passes if there are two green boxes and no red.

<div id="dialog-parent">
    <dialog id="dialog"></dialog>
</div>
<div id="backdrop-ancestor">
    <p><span><dialog></dialog></span></p>
</div>
<script>
var dialogs = document.querySelectorAll('dialog');
for (var i = 0; i < dialogs.length; ++i)
    dialogs[i].showModal();
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/backdrop-descendant-selector.html"
}
```
