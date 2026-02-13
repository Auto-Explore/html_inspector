# html/browsers/browsing-the-web/history-traversal/persisted-user-state-restoration/resources/page-with-fragment.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/history-traversal/persisted-user-state-restoration/resources/page-with-fragment.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<style>
body {
    height: 2000px;
    width: 2000px;
}
#fragment {
    position: absolute;
    top: 800px;
    background-color: #faa;
    display: block;
    height: 100px;
    width: 100px;
}

</style>
<body>
Page with fragment
  <a id="fragment" name="fragment" class='box'></a>
</body>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.a.name.obsolete",
      "message": "The “name” attribute on the “a” element is obsolete. Consider putting an “id” attribute on the nearest container instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 294,
        "byte_start": 249,
        "col": 3,
        "line": 19
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
  "source_name": "html/browsers/browsing-the-web/history-traversal/persisted-user-state-restoration/resources/page-with-fragment.html"
}
```
