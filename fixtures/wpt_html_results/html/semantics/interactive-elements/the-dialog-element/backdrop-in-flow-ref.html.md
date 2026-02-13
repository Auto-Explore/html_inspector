# html/semantics/interactive-elements/the-dialog-element/backdrop-in-flow-ref.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/backdrop-in-flow-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<style>
#backdrop {
    position: absolute;
    top: 100px;
    left: 100px;
    height: 100px;
    width: 100px;
    background: green;
}
</style>
<body>
Test that position 'static' or 'relative' for ::backdrop computes to 'absolute'.
The test passes if there is a single green box.
<div id="backdrop"></div>
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/backdrop-in-flow-ref.html"
}
```
