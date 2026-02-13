# html/rendering/replaced-elements/the-select-element/select-1-block-size-001.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/replaced-elements/the-select-element/select-1-block-size-001.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Select block size when line-height is specified</title>
<link rel="author" title="Emilio Cobos Álvarez" href="mailto:emilio@crisal.io">
<link rel="author" title="Mozilla" href="https://mozilla.org">
<link rel="help" href="https://bugzilla.mozilla.org/show_bug.cgi?id=1571764">
<link rel="match" href="select-1-block-size-001-ref.html">
<style>
select {
  -webkit-appearance: none;
  appearance: none;

  background: black;
  color: black;

  line-height: 100px;
  width: 100px;

  border: 0;
  border-radius: 0;
  padding: 0;
}
</style>
<select></select>
<select><option>A</option></select>
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
  "source_name": "html/rendering/replaced-elements/the-select-element/select-1-block-size-001.html"
}
```
