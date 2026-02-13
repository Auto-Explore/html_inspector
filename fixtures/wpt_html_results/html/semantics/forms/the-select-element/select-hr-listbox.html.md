# html/semantics/forms/the-select-element/select-hr-listbox.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/select-hr-listbox.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<link rel="help" href="https://bugzilla.mozilla.org/show_bug.cgi?id=2008003">
<link rel="author" href="mailto:emilio@crisal.io" title="Emilio Cobos Álvarez">
<link rel="author" href="https://mozilla.org" title="Mozilla">
<link rel="mismatch" href="select-hr-listbox-notref.html">
<title>hr renders in listbox select</title>
<select size="10">
  <option>A</option>
  <hr>
  <option>B</option>
  <option>C</option>
</select>
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
  "source_name": "html/semantics/forms/the-select-element/select-hr-listbox.html"
}
```
