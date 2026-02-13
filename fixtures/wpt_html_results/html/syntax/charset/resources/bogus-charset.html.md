# html/syntax/charset/resources/bogus-charset.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/syntax/charset/resources/bogus-charset.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html><meta charset=this-is-not-a-charset><script>
onload = () => {
  if (location.search === "?postMessage") {
    parent.postMessage(document.body.textContent, "*");
  }
}
</script>�
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.meta.charset.mismatch",
      "message": "Internal encoding declaration “this-is-not-a-charset” disagrees with the actual encoding of the document (“utf-8”).",
      "severity": "Warning",
      "span": {
        "byte_end": 51,
        "byte_start": 15,
        "col": 16,
        "line": 1
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
  "source_name": "html/syntax/charset/resources/bogus-charset.html"
}
```
