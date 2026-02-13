# html/semantics/interactive-elements/the-dialog-element/backdrop-in-flow.html

Counts:
- errors: 1
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/backdrop-in-flow.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel="match" href="backdrop-in-flow-ref.html">
<link rel="help" href="https://fullscreen.spec.whatwg.org/#new-stacking-layer">
<style>
dialog {
    visibility: hidden;
}

dialog::backdrop {
    visibility: visible;
    height: 100px;
    width: 50px;
}

#left::backdrop {
    position: static;
    top: 100px;
    left: 100px;
    background: green;
}

#right::backdrop {
    position: relative;
    background: green;
    top: 100px;
    left: 150px;
}
</style>
<body>
Test that position 'static' or 'relative' for ::backdrop computes to 'absolute'.
The test passes if there is a single green box.
<dialog id="left"></dialog>
<dialog id="right"></dialog>
</div>
<script>
document.querySelector('#left').showModal();
document.querySelector('#right').showModal();
</script>
</body>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “div”.",
      "severity": "Error",
      "span": {
        "byte_end": 683,
        "byte_start": 677,
        "col": 1,
        "line": 34
      }
    },
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/backdrop-in-flow.html"
}
```
