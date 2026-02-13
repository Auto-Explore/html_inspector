# html/semantics/interactive-elements/the-dialog-element/top-layer-stacking-correct-order-remove-readd-ref.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/top-layer-stacking-correct-order-remove-readd-ref.html",
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
    height: 100px;
    width: 100px;
}
</style>
</head>
<body>
<p>Bug <a href="https://bugs.webkit.org/show_bug.cgi?id=105489">105489</a>: Elements must be reattached when inserted/removed from top layer
<p>The test passes if you see a green rectangle stacked on top of a blue rectangle.

<div class="pseudodialog" style="top: 100px; background-color: blue"></div>
<div class="pseudodialog" style="top: 150px; left: 50px; background-color: green"></div>
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/top-layer-stacking-correct-order-remove-readd-ref.html"
}
```
