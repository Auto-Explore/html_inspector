# html/semantics/forms/the-datalist-element/remove-datalist-crash.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-datalist-element/remove-datalist-crash.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Remove datalist element crash</title>
<link rel="help" href="https://crbug.com/1199861">
<datalist id="datalist"><option>foo</option></datalist>
<input id="input">
<input list="datalist">
<script>
  document.body.offsetTop;
  input.appendChild(datalist);
  datalist.remove();
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
  "source_name": "html/semantics/forms/the-datalist-element/remove-datalist-crash.html"
}
```
