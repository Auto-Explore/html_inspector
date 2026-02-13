# html/rendering/replaced-elements/the-option-element/select-multiple-covered-by-abspos.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/replaced-elements/the-option-element/select-multiple-covered-by-abspos.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<link rel="help" href="https://bugzilla.mozilla.org/show_bug.cgi?id=1770532">
<link rel="author" href="mailto:dholbert@mozilla.com" title="Daniel Holbert">
<link rel="author" href="https://mozilla.org" title="Mozilla">
<link rel="match" href="select-multiple-covered-by-abspos-ref.html">
<title>Combobox selects are not stacking contexts by default</title>
<style>
.abspos {
  position: absolute;
  background-color: green;
  height: 300px;
  width: 300px;
}
</style>
<div class="abspos"></div>
<select multiple>
  <option>This text shouldn't be visible.</option>
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
  "source_name": "html/rendering/replaced-elements/the-option-element/select-multiple-covered-by-abspos.html"
}
```
