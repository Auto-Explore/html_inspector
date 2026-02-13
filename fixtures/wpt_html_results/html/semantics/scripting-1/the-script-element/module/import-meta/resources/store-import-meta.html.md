# html/semantics/scripting-1/the-script-element/module/import-meta/resources/store-import-meta.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/module/import-meta/resources/store-import-meta.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<script type="module">
window.importMetaURL = import.meta.url;
window.importMetaResolve = import.meta.resolve;
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
  "source_name": "html/semantics/scripting-1/the-script-element/module/import-meta/resources/store-import-meta.html"
}
```
