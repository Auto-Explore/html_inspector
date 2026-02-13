# html/rendering/non-replaced-elements/form-controls/input-placeholder-line-height.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/form-controls/input-placeholder-line-height.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>line-height has no effect on placeholder</title>
<link rel="help" href="https://bugzilla.mozilla.org/show_bug.cgi?id=1714631">
<link rel="match" href="input-placeholder-line-height-ref.html">
<style>
  input::placeholder {
    line-height: 0;
  }
</style>
<input placeholder=Foo>
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
  "source_name": "html/rendering/non-replaced-elements/form-controls/input-placeholder-line-height.html"
}
```
