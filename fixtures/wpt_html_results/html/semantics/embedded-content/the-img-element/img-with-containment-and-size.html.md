# html/semantics/embedded-content/the-img-element/img-with-containment-and-size.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/img-with-containment-and-size.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html class="reftest-wait">
<title>Ensure images with containment and size are rendered properly</title>
<meta charset="utf-8">
<link rel="match" href="img-with-containment-and-size-ref.html">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#the-img-element">
<style>
img {
  contain: paint;
  width: 200px;
  height: 100px;
  will-change: transform;
}
</style>
<script>
onload = () => {
  var i = new Image();
  i.onload = function() {
    document.body.appendChild(i);
    document.documentElement.classList.remove("reftest-wait");
  };
  i.src = "image.png";
};
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
  "source_name": "html/semantics/embedded-content/the-img-element/img-with-containment-and-size.html"
}
```
