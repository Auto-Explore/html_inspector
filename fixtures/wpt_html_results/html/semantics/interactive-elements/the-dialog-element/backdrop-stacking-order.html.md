# html/semantics/interactive-elements/the-dialog-element/backdrop-stacking-order.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/backdrop-stacking-order.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel="match" href="backdrop-stacking-order-ref.html">
<link rel="help" href="https://fullscreen.spec.whatwg.org/#new-stacking-layer">
<style>
dialog {
    padding: 0px;
    border: none;
    margin: 0px;
    outline: none;
}

#bottom::backdrop {
    top: 100px;
    left: 100px;
    height: 300px;
    width: 300px;
    background-color: rgb(0, 50, 0);
    z-index: 100;  /* z-index has no effect. */
}

#bottom {
    top: 125px;
    left: 125px;
    height: 250px;
    width: 250px;
    background-color: rgb(0, 90, 0);
}

#middle::backdrop {
    top: 150px;
    left: 150px;
    height: 200px;
    width: 200px;
    background-color: rgb(0, 130, 0);
    z-index: -100;  /* z-index has no effect. */
}

#middle {
    top: 175px;
    left: 175px;
    height: 150px;
    width: 150px;
    background-color: rgb(0, 170, 0);
}

#top::backdrop {
    top: 200px;
    left: 200px;
    height: 100px;
    width: 100px;
    background-color: rgb(0, 210, 0);
    z-index: 0;  /* z-index has no effect. */
}

#top {
    top: 225px;
    left: 225px;
    height: 50px;
    width: 50px;
    background-color: rgb(0, 255, 0);
    z-index: -1000;  /* z-index has no effect. */
}
</style>
<body>
Test for dialog::backdrop stacking order. The test passes if there are 6
boxes enclosed in each other, becoming increasingly smaller and brighter
green.
<dialog id="top"></dialog>
<dialog id="middle"></dialog>
<dialog id="bottom"></dialog>
<script>
var topDialog = document.getElementById('top');
var middleDialog = document.getElementById('middle');
var bottomDialog = document.getElementById('bottom');
topDialog.showModal();
bottomDialog.showModal();
topDialog.close();  // Just to shuffle the top layer order around a little.
middleDialog.showModal();
topDialog.showModal();
</script>
</body>
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/backdrop-stacking-order.html"
}
```
