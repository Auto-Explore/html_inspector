# html/semantics/selectors/pseudo-classes/indeterminate-radio-group-ref.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/selectors/pseudo-classes/indeterminate-radio-group-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<title>Test reference</title>
<style>
label[for="two"] {
  background: green;
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
  "source_name": "html/semantics/selectors/pseudo-classes/indeterminate-radio-group-ref.html"
}
```
