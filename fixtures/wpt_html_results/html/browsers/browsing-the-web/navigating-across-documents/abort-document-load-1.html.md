# html/browsers/browsing-the-web/navigating-across-documents/abort-document-load-1.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/navigating-across-documents/abort-document-load-1.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<script>
parent.postMessage(document.readyState, "*");
let f = document.createElement("iframe");
f.onload = function() {
  parent.postMessage("stop", "*");
  window.stop();
};
document.documentElement.appendChild(f);

window.addEventListener("load", (event) => {
    parent.postMessage("load", "*");
});
window.addEventListener("error", (event) => {
    parent.postMessage("error", "*");
});
window.addEventListener("abort", (event) => {
    parent.postMessage("abort", "*");
});
window.addEventListener("pageshow", (event) => {
    parent.postMessage("pageshow", "*");
});
window.addEventListener("DOMContentLoaded", (event) => {
    parent.postMessage("DOMContentLoaded", "*");
});
document.addEventListener("readystatechange", (event) => {
    if (document.readyState === "complete") {
        parent.postMessage("complete", "*");
    }
});

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
  "source_name": "html/browsers/browsing-the-web/navigating-across-documents/abort-document-load-1.html"
}
```
