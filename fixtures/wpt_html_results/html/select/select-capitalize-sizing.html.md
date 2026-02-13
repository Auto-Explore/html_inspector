# html/select/select-capitalize-sizing.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/select/select-capitalize-sizing.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<title>select sizing with text-transform:capitalize</title>
<link rel="match" href="select-capitalize-sizing-ref.html">
<link rel="help" href="https://bugzilla.mozilla.org/show_bug.cgi?id=1911550">
<style>
select {
  text-transform: capitalize;
}
.appearance-none {
  appearance: none;
  padding: 0;
  border: 1px solid;
}
</style>
<select>
  <option>x x x x x</option>
</select>
<br><br>
<select class=appearance-none>
  <option>x x x x x</option>
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
  "source_name": "html/select/select-capitalize-sizing.html"
}
```
