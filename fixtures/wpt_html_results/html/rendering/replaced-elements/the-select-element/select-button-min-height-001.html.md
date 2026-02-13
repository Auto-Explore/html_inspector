# html/rendering/replaced-elements/the-select-element/select-button-min-height-001.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/replaced-elements/the-select-element/select-button-min-height-001.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>min-height and height both trigger same rendering for select and buttons by default</title>
<link rel="author" title="Emilio Cobos Álvarez" href="mailto:emilio@crisal.io">
<link rel="author" title="Mozilla" href="https://mozilla.org">
<link rel="help" href="https://bugzilla.mozilla.org/show_bug.cgi?id=1362907">
<link rel="match" href="select-button-min-height-001-ref.html">
<style>
  select, input, button { min-height: 100px; }
</style>
<div>
  <select><option>Select</option></select>
  <button>Button</button>
  <input type="button" value="Input">
</div>
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
  "source_name": "html/rendering/replaced-elements/the-select-element/select-button-min-height-001.html"
}
```
