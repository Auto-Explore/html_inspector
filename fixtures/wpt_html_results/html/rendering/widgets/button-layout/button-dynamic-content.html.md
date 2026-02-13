# html/rendering/widgets/button-layout/button-dynamic-content.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/widgets/button-layout/button-dynamic-content.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel="help" href="https://issues.chromium.org/issues/353898969">
<link rel="match" href="button-dynamic-content-ref.html">
<button style="width: 100px; height: 100px;">
  text text text text text text text text
  <span id="target"></span>
</button>
<script>
document.body.offsetTop;
document.getElementById('target').innerText = "text";
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.head.title.missing",
      "message": "Element “head” is missing a required instance of child element “title”.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/rendering/widgets/button-layout/button-dynamic-content.html"
}
```
