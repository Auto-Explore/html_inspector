# html/editing/editing-0/contenteditable/contenteditable-overflow-height-ref.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/editing-0/contenteditable/contenteditable-overflow-height-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>CSS test reference</title>
<style>
  [contenteditable] {
    outline: 1px solid black;
    outline-offset: -1px;
  }
</style>
<div contenteditable></div>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/editing/editing-0/contenteditable/contenteditable-overflow-height-ref.html"
}
```
