# html/semantics/interactive-elements/the-dialog-element/top-layer-nesting-ref.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/top-layer-nesting-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
<style>
.pseudodialog {
    height: 150px;
    width: 150px;
    position: absolute;
    top: 0; right: 0; bottom: 0; left: 0;
    margin: auto;
    border: solid;
    padding: 1em;
    background: white;
    color: black;
}
</style>
</head>
<body>
This tests that top layer elements are stacked correctly even if nested in the DOM tree.
The test passes if you see no red rectangles and see 3 rectangles stacked in the following order (from bottom to top): yellow, blue, green.

<div class="pseudodialog" style="top: 100px; background-color: yellow"></div>
<div class="pseudodialog" style="top: 150px; left: 50px; background-color: blue"></div>
<div class="pseudodialog" style="top: 200px; left: 100px; background-color: green"></div>
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/top-layer-nesting-ref.html"
}
```
