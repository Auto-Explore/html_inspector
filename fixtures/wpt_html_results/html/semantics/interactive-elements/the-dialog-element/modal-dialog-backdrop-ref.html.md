# html/semantics/interactive-elements/the-dialog-element/modal-dialog-backdrop-ref.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/modal-dialog-backdrop-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<style>
.dialog-default-ua-style {
    position: absolute;
    overflow: auto;
    top: 0;
    right: 0;
    bottom: 0;
    left: 0;
    margin: auto;
    border: solid;
    padding: 1em;
    background: white;
    color: black;
}

#dialog {
    margin: auto;
    height: 100px;
    width: 100px;
    top: 100px;
    z-index: 1000;
    background: green;
}

#backdrop {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0,0,0,0.1);
    z-index: 100;
}
</style>
<body>
Test for the default user agent style of dialog::backdrop. The test passes if
there is a green box, above a very lightly translucent gray box spanning the
viewport.
<div id="backdrop"></div>
<div class="dialog-default-ua-style" id="dialog"></div>
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/modal-dialog-backdrop-ref.html"
}
```
