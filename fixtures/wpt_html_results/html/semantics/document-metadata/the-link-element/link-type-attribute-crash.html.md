# html/semantics/document-metadata/the-link-element/link-type-attribute-crash.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/document-metadata/the-link-element/link-type-attribute-crash.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=help href="https://github.com/servo/servo/issues/42259">
<link id="link" rel="stylesheet" href="data:text/css,div { background-color: green !important; }">
<script>
  link.type = "text/css";
  link.type = "text/css";
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.link.href.invalid",
      "message": "Bad value “data:text/css,div { background-color: green !important; }” for attribute “href” on element “link”.",
      "severity": "Warning",
      "span": {
        "byte_end": 181,
        "byte_start": 83,
        "col": 1,
        "line": 3
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
  "source_name": "html/semantics/document-metadata/the-link-element/link-type-attribute-crash.html"
}
```
