# html/semantics/embedded-content/the-iframe-element/iframe-document-move-crash.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/iframe-document-move-crash.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
This test passes if it does not crash.
<script>
  var doc1 = document.documentElement;
  let iframe1 = document.createElement("iframe");
  doc1.appendChild(iframe1);
  separateDoc = document.implementation.createDocument("", null);
  iframe1.addEventListener("DOMFocusOut", function () {  separateDoc.adoptNode(iframe1); });
  iframe1.focus();
  iframe1 = document.createElement("iframe");
  doc1.appendChild(iframe1);
  iframe1.focus();
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
  "source_name": "html/semantics/embedded-content/the-iframe-element/iframe-document-move-crash.html"
}
```
