# html/webappapis/dynamic-markup-insertion/document-write/resources/mutation-observer-iframe-script-async.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/dynamic-markup-insertion/document-write/resources/mutation-observer-iframe-script-async.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<script>
new MutationObserver((mutations, observer) => {
  mutations.forEach(({ addedNodes }) => {
    addedNodes.forEach(node => {
      if (node.nodeType == 1 && node.tagName == "SCRIPT") {
        document.write("document.write body contents\n");
        document.close();
        window.parent.document.dispatchEvent(new CustomEvent("documentWriteDone"));
      }
    });
  });
}).observe(document.documentElement, { childList: true, subtree: true });
</script>
Initial body contents
<script src="mutation-observer-iframe-script.js" async></script>
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
  "source_name": "html/webappapis/dynamic-markup-insertion/document-write/resources/mutation-observer-iframe-script-async.html"
}
```
