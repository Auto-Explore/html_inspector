# html/browsers/browsing-the-web/navigating-across-documents/resources/unknown-protocol-reload-crash-frame.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/navigating-across-documents/resources/unknown-protocol-reload-crash-frame.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<script>
  (async function() {
    if (!location.hash) {
      await new Promise(r => {
        window.addEventListener("hashchange", r, { once: true });
        location.hash = "foo";
      });
      location.reload(true);
    } else {
      location.href = "unlikely-protocol://foo"
      parent.document.documentElement.classList = "";
    }
  })();
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
  "source_name": "html/browsers/browsing-the-web/navigating-across-documents/resources/unknown-protocol-reload-crash-frame.html"
}
```
