# html/rendering/widgets/appearance/appearance-animation-002.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/widgets/appearance/appearance-animation-002.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>compute the kind of widget: animation origin only</title>
<link rel="match" href="appearance-animation-002-ref.html">
<style>
  .bg200 {
    animation: 1e10s steps(2, start) animate-bg;
  }
  @keyframes animate-bg {
    to {
      background-color: rgb(255, 0, 0);
    }
  }
</style>
<input value="text" id=input>
<script>
  document.documentElement.offsetTop;
  input.classList.toggle('bg200');
</script>
<p>PASS if the input field does not have a red background</p>
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
  "source_name": "html/rendering/widgets/appearance/appearance-animation-002.html"
}
```
