# html/semantics/embedded-content/the-img-element/adopt-from-image-document.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/adopt-from-image-document.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Adopt img from image document</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/#the-img-element">
<link rel="match" href="document-base-url-ref.html">
<!-- Counteract any style added by the image document -->
<style>img { width: initial; height: initial; }</style>
<iframe></iframe>
<script>
  var iframe = document.querySelector('iframe');
  iframe.onload = function() {
    let img = iframe.contentDocument.body.firstChild;
    document.body.appendChild(img);
    iframe.remove();
  };
  iframe.src = 'resources/cat.jpg';
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
  "source_name": "html/semantics/embedded-content/the-img-element/adopt-from-image-document.html"
}
```
