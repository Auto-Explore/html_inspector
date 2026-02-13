# html/rendering/widgets/the-select-element/select-size-002.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/widgets/the-select-element/select-size-002.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>select size=0 renders the same as plain select</title>
<link rel="help" href="https://bugzilla.mozilla.org/show_bug.cgi?id=1643279">
<link rel="author" title="Mozilla" href="https://mozilla.org">
<link rel="author" title="Emilio Cobos Álvarez" href="mailto:emilio@crisal.io">
<link rel="match" href="select-size-ref.html">
<select size="0">
  <option value ="1">1</option>
  <option value ="2">2</option>
  <option value ="3">3</option>
</select>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.select.size.nonzero",
      "message": "Bad value “0” for attribute “size” on element “select”.",
      "severity": "Warning",
      "span": {
        "byte_end": 364,
        "byte_start": 347,
        "col": 1,
        "line": 7
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/rendering/widgets/the-select-element/select-size-002.html"
}
```
