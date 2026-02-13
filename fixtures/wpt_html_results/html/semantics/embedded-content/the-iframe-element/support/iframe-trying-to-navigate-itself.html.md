# html/semantics/embedded-content/the-iframe-element/support/iframe-trying-to-navigate-itself.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/support/iframe-trying-to-navigate-itself.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<p>If this message appears, then this frame has not been navigated.</p>
<script>
window.onload = function() {
  try {
    location.href = "data:text/html,\u003c!DOCTYPE html\u003e\u003cp\u003eIf this message appears, then this frame has been navigated.\u003c/p\u003e\u003cscript\u003eparent.postMessage('can navigate', '*');\u003c/script\u003e";
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
  "source_name": "html/semantics/embedded-content/the-iframe-element/support/iframe-trying-to-navigate-itself.html"
}
```
