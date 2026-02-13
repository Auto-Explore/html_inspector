# html/rendering/widgets/input-checkbox-zero-size.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/widgets/input-checkbox-zero-size.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<link rel="help" href="https://bugzilla.mozilla.org/show_bug.cgi?id=1903191">
<link rel="author" href="mailto:emilio@crisal.io" title="Emilio Cobos Álvare">
<link rel="author" href="https://mozilla.org" title="Mozilla">
<link rel="match" href="/css/reference/blank.html">
<style>
  input { width: 0; height: 0 }
</style>
<input type=checkbox>
<input type=radio>
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
  "source_name": "html/rendering/widgets/input-checkbox-zero-size.html"
}
```
