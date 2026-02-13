# html/semantics/embedded-content/the-img-element/available-images.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/available-images.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html class="reftest-wait">
<title>Ensure images from available images list are rendered</title>
<meta charset="utf-8">
<link rel="match" href="available-images-ref.html">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#the-img-element">
<div id="log"></div>
<script>
  var i = new Image();
  i.onload = function() {
    var i2 = new Image();
    i2.src = "3.jpg";
    document.body.appendChild(i2);
    document.documentElement.classList.remove("reftest-wait");
  };
  i.src = "3.jpg";
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
  "source_name": "html/semantics/embedded-content/the-img-element/available-images.html"
}
```
