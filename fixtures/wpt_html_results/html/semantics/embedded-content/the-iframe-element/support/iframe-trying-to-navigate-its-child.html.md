# html/semantics/embedded-content/the-iframe-element/support/iframe-trying-to-navigate-its-child.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/support/iframe-trying-to-navigate-its-child.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<iframe src="data:text/html,If this message appears, then this frame has not been navigated by its parent."></iframe>
<script>
window.onload = function() {
  try {
    document.querySelector("iframe").contentWindow.location.href = "data:text/html,\u003c!DOCTYPE html\u003e\u003cp\u003eIf this message appears, then this frame has been navigated by its parent.\u003c/p\u003e\u003cscript\u003eparent.parent.postMessage('can navigate', '*');\u003c/script\u003e";
  } catch(e) {
    parent.postMessage("can not navigate", "*");
  }
}
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.iframe.src.invalid",
      "message": "Bad value “data:text/html,If this message appears, then this frame has not been navigated by its parent.” for attribute “src” on element “iframe”.",
      "severity": "Warning",
      "span": {
        "byte_end": 124,
        "byte_start": 16,
        "col": 1,
        "line": 2
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
  "source_name": "html/semantics/embedded-content/the-iframe-element/support/iframe-trying-to-navigate-its-child.html"
}
```
