# html/semantics/links/links-created-by-a-and-area-elements/support/noopener-popup.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/links/links-created-by-a-and-area-elements/support/noopener-popup.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<script>
  function findLink(arg) {
    var doc;
    if (arg == "self") {
      doc = document;
    } else {
      doc = frames[0].document;
    }
    return doc.getElementById(arg + "target");
  }
</script>
<a rel="noopener" target="_self" id="selftarget"
   href="noopener-target-1.html"></a>
<iframe srcdoc='
        <a rel="noopener" target="_parent" id="parenttarget"
           href="noopener-target-1.html"></a>
        <a rel="noopener" target="_top" id="toptarget"
           href="noopener-target-1.html"></a>'></iframe>
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
  "source_name": "html/semantics/links/links-created-by-a-and-area-elements/support/noopener-popup.html"
}
```
