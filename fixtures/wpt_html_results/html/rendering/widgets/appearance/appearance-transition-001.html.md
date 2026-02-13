# html/rendering/widgets/appearance/appearance-transition-001.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/widgets/appearance/appearance-transition-001.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>compute the kind of widget: author origin and transition origin</title>
<link rel="help" href="https://crbug.com/1086732">
<link rel="match" href="appearance-transition-001-ref.html">
<style>
  input {
    background-color: rgb(0, 0, 0);
    transition: background-color 1e10s steps(2, start);
  }

  .bg200 {
    background-color: rgb(0, 200, 0);
  }
</style>
<input value="text" id=input>
<script>
  document.documentElement.offsetTop;
  input.classList.toggle('bg200');
</script>
<p>PASS if the input field has a dark green background</p>
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
  "source_name": "html/rendering/widgets/appearance/appearance-transition-001.html"
}
```
