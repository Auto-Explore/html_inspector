# html/semantics/embedded-content/the-img-element/image-loading-lazy-subframe-detached-crash.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/image-loading-lazy-subframe-detached-crash.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html class="test-wait">
<title>Crash when detaching a frame during a lazy-load operation</title>
<link rel="author" href="mailto:emilio@crisal.io" title="Emilio Cobos Álvarez">
<link rel="author" href="https://mozilla.org" title="Mozilla">
<link rel="help" href="https://bugzilla.mozilla.org/show_bug.cgi?id=1619858">
<iframe srcdoc=""></iframe>
<script>
onload = function() {
  let frame = document.querySelector("iframe");
  frame.contentDocument.body.innerHTML = `
    <div style="height: 300vh"></div>
    <img loading="lazy" src="/images/blue96x96.png" width=96 height=96>
  `;
  let img = frame.contentDocument.querySelector("img");
  new IntersectionObserver(() => {
    frame.remove();
    requestAnimationFrame(function() {
      requestAnimationFrame(function() {
        document.documentElement.className = "";
      });
    });
  }).observe(img);
  frame.contentWindow.scrollTo(0, img.getBoundingClientRect().top);
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
  "source_name": "html/semantics/embedded-content/the-img-element/image-loading-lazy-subframe-detached-crash.html"
}
```
