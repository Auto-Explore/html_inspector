# html/semantics/forms/the-select-element/select-listbox-selected-hscroll.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/select-listbox-selected-hscroll.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>Listbox selected option should be visible after horizontal scroll</title>
<link rel="author" title="Emilio Cobos Álvarez" href="mailto:emilio@crisal.io">
<link rel="author" title="Mozilla" href="https://mozilla.com">
<link rel="help" href="https://bugzilla.mozilla.org/show_bug.cgi?id=2007620">
<link rel="mismatch" href="/css/reference/blank.html">
<style>
#s {
  box-sizing: content-box;
  width: 1ch;
  font: monospace;
  overflow: scroll;
  outline: none;
  border: none;
  scrollbar-width: none;
}
</style>
<select id="s" size="2">
  <option selected>AAAAA</option>
</select>
<script>
  s.focus();
  s.scrollLeft = 100000;
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
  "source_name": "html/semantics/forms/the-select-element/select-listbox-selected-hscroll.html"
}
```
