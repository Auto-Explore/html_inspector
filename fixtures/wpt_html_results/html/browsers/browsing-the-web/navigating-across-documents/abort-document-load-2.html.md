# html/browsers/browsing-the-web/navigating-across-documents/abort-document-load-2.html

Counts:
- errors: 1
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/navigating-across-documents/abort-document-load-2.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<script>
parent.postMessage(document.readyState, "*");

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

window.setTimeout(function() {
  parent.postMessage("stop", "*");
  window.stop();
}, 100);

</script>
<link rel="stylesheet" href="/common/slow.py"></link>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “link”.",
      "severity": "Error",
      "span": {
        "byte_end": 855,
        "byte_start": 848,
        "col": 47,
        "line": 32
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
  "source_name": "html/browsers/browsing-the-web/navigating-across-documents/abort-document-load-2.html"
}
```
