# html/semantics/interactive-elements/the-dialog-element/resources/inert-focus-in-frames-frame1.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/resources/inert-focus-in-frames-frame1.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
<script>
window.onload = parent.parent.frameLoaded;
</script>
</head>
<body>
<dialog id="dialog">
    <button id="dialog-button" tabindex="0">Button</button>
    <iframe id="iframe-in-dialog" srcdoc='
        <input id="iframe-under-dialog-input" class="target" type="date">
    '></iframe>
</dialog>
<input id="frame1-input" class="target" type="text">
<iframe id="iframe1" srcdoc='
    <dialog id="iframe-dialog">
        <button id="iframe-dialog-button" tabindex="0">Button</button>
    </dialog>
    <input id="iframe-input" class="target" type="date">
    <script>window.onload = parent.parent.parent.frameLoaded;</script>
'>
</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.iframe.text.disallowed",
      "message": "Text not allowed in “iframe” in this context.",
      "severity": "Warning",
      "span": null
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/resources/inert-focus-in-frames-frame1.html"
}
```
