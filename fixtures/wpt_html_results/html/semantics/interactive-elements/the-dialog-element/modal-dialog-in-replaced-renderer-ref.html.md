# html/semantics/interactive-elements/the-dialog-element/modal-dialog-in-replaced-renderer-ref.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/modal-dialog-in-replaced-renderer-ref.html",
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
dialog {
    background: green;
    border-color: green;
}
div {
    content: url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAA8AAAAPAQMAAAABGAcJAAAAA1BMVEUAgACc+aWRAAAADElEQVR42mNgIAEAAAAtAAH7KhMqAAAAAElFTkSuQmCC);
}
</style>
</head>
<body>
<p>The test passes if you see a green square near the top of the viewport.
<div></div>
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/modal-dialog-in-replaced-renderer-ref.html"
}
```
