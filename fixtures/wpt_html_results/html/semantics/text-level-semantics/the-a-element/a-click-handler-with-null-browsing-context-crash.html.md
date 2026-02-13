# html/semantics/text-level-semantics/the-a-element/a-click-handler-with-null-browsing-context-crash.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/text-level-semantics/the-a-element/a-click-handler-with-null-browsing-context-crash.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>HTMLAnchorElement.onclick with null browsing context</title>
<link rel="author" title="Nate Chapin" href="mailto:japhet@chromium.org">
<link rel="help" href="https://html.spec.whatwg.org/C/#the-a-element">
<link rel="help" href="https://bugs.chromium.org/p/chromium/issues/detail?id=1083721">
<meta name="assert" content="An <a> from a discarded browsing context that is cloned into a new document should not crash when clicked"/>
<body>
<iframe id="i" src="resources/a-onclick-handler-iframe.html"></iframe>
<script>
window.onload = () => {
  var range = i.contentDocument.createRange();
  range.selectNodeContents(i.contentDocument.body);
  i.remove();

  // Clone the <a> into this document, and ensure clicking it does not crash.
  document.body.appendChild(range.cloneContents());
  var a = document.getElementsByTagName('a')[0];
  a.click();
};
</script>
</body>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/text-level-semantics/the-a-element/a-click-handler-with-null-browsing-context-crash.html"
}
```
