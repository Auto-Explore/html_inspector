# html/editing/editing-0/contenteditable/contenteditable-overflow-height.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/editing-0/contenteditable/contenteditable-overflow-height.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Overflow still allows contenteditable elements to have height</title>
<link rel="author" title="Emilio Cobos Álvarez" href="mailto:emilio@crisal.io">
<link rel="author" title="Mozilla" href="https://mozilla.org">
<link rel="help" href="https://bugzilla.mozilla.org/show_bug.cgi?id=1681375">
<link rel="match" href="contenteditable-overflow-height-ref.html">
<style>
  [contenteditable] {
    outline: 1px solid black;
    outline-offset: -1px;
    overflow: hidden;
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
  "source_name": "html/editing/editing-0/contenteditable/contenteditable-overflow-height.html"
}
```
