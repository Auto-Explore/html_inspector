# html/semantics/selectors/pseudo-classes/indeterminate-radio-group.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/selectors/pseudo-classes/indeterminate-radio-group.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<title>:indeterminate and input type=radio</title>
<link rel="author" href="mailto:emilio@crisal.io" title="Emilio Cobos Álvarez">
<link rel="author" href="https://mozilla.org" title="Mozilla">
<link rel="help" href="https://bugzilla.mozilla.org/show_bug.cgi?id=1861346">
<link rel="match" href="indeterminate-radio-group-ref.html">
<style>
input:checked + label {
  background: green;
}
input:indeterminate + label {
  background: red;
}
</style>
<form>
  <input type="radio" name="a" id="one" value="1">
  <label for="one">One</label>

  <input type="radio" name="a" id="two" value="2" checked>
  <label for="two">Two</label>

  <input type="radio" name="a" id="three" value="3">
  <label for="three">Three</label>

  <input type="radio" name="a" id="four" value="4">
  <label for="four">Four</label>
</form>
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
  "source_name": "html/semantics/selectors/pseudo-classes/indeterminate-radio-group.html"
}
```
