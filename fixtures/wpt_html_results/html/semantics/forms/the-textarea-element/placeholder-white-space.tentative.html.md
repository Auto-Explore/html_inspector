# html/semantics/forms/the-textarea-element/placeholder-white-space.tentative.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-textarea-element/placeholder-white-space.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<title>Textarea placeholder honors textarea's text-overflow</title>
<link rel=author href="mailto:emilio@crisal.io" title="Emilio Cobos Álvarez">
<link rel=author href="https://mozilla.com" title="Mozilla">
<link rel=mismatch href="placeholder-white-space-notref.html">
<link rel=help href="https://github.com/w3c/csswg-drafts/issues/6669">
<style>
  textarea {
    white-space: nowrap;
    text-overflow: ellipsis;
    max-width: 100px;
  }
</style>
<textarea placeholder="This is a really long string that needs to be truncated"></textarea>
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
  "source_name": "html/semantics/forms/the-textarea-element/placeholder-white-space.tentative.html"
}
```
