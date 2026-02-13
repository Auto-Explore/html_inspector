# html/semantics/forms/the-select-element/select-innerHTML-selected.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/select-innerHTML-selected.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<title>Parser insertion via innerHTML with selected attribute</title>
<link rel="author" title="Emilio Cobos Álvarez" href="mailto:emilio@crisal.io">
<link rel="author" title="Mozilla" href="https://mozilla.com">
<link rel="help" href="https://bugzilla.mozilla.org/show_bug.cgi?id=2014604">
<link rel="match" href="select-innerHTML-selected-ref.html">
<div id="target"></div>
<script>
onload = () => {
  target.innerHTML = `
    <select><option selected>ABC</option></select>
    <select><option>ABC</option></select>
  `;
};
</script>
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
  "source_name": "html/semantics/forms/the-select-element/select-innerHTML-selected.html"
}
```
