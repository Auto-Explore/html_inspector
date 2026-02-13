# html/semantics/interactive-elements/the-dialog-element/top-layer-stacking-dynamic-ref.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/top-layer-stacking-dynamic-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
<link rel="stylesheet" href="resources/dialog.css">
<style>
.pseudodialog {
    height: 150px;
    width: 150px;
}
</style>
</head>
<body>
This tests top layer element stacking order after dynamically calling show/close and removal from the DOM tree.
The test passes if you see a green rectangle stacked on top of a blue rectangle, and see no red rectangles.

<div class="pseudodialog" style="top: 50px; background-color: blue"></div>
<div class="pseudodialog" style="top: 100px; left: 50px; background-color: green"></div>
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/top-layer-stacking-dynamic-ref.html"
}
```
