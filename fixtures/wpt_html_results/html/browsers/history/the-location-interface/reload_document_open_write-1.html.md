# html/browsers/history/the-location-interface/reload_document_open_write-1.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/history/the-location-interface/reload_document_open_write-1.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
1
<script>
function f() {
  opener.postMessage("original", "*");
  if (opener.data.length >= 2) {
    // If we proceed here, then our document.write will be racing with the
    // setTimeout in our opener.  Just stop.
    return;
  }
  setTimeout(function () {
    document.open();
    document.write("<!doctype html>2<script>opener.postMessage('written', '*');<\/script>");
    document.close();
  });
}

window.onload = f
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
  "source_name": "html/browsers/history/the-location-interface/reload_document_open_write-1.html"
}
```
