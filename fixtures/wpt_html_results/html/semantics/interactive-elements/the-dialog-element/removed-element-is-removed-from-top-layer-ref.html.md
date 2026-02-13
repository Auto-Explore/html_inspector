# html/semantics/interactive-elements/the-dialog-element/removed-element-is-removed-from-top-layer-ref.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/removed-element-is-removed-from-top-layer-ref.html",
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

#bottomDialog {
    background-color: blue;
    top: 0px;
}

#topDialog {
    background-color: green;
    top: 50px;
    left: 50px;
}
</style>
</head>
<body>
<p>Bug <a href="https://bugs.webkit.org/show_bug.cgi?id=105489">105489</a>: Elements must be reattached when inserted/removed from top layer
<p>The test passes if you see a green rectangle stacked on top of a blue rectangle.
<div id="bottomDialog" class="pseudodialog"></div>
<div id="topDialog" class="pseudodialog"></div>
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/removed-element-is-removed-from-top-layer-ref.html"
}
```
