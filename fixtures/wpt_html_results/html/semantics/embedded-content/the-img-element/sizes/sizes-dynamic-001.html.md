# html/semantics/embedded-content/the-img-element/sizes/sizes-dynamic-001.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/sizes/sizes-dynamic-001.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html class="reftest-wait">
<title>Image intrinsic size specified via sizes attribute reacts properly to media changes</title>
<link rel="author" title="Emilio Cobos Álvarez" href="mailto:emilio@crisal.io">
<link rel="match" href="sizes-dynamic-001-ref.html">
<link rel="help" href="https://html.spec.whatwg.org/#sizes-attributes">
<link rel="help" href="https://bugzilla.mozilla.org/show_bug.cgi?id=1149357">
<script>
function frameLoaded(frame) {
  frame.width = "500";
  let img = frame.contentDocument.querySelector('img');

  // Trigger the viewport resize, which will trigger the image load task.
  img.offsetWidth;

  // Wait for the image load task to run.
  setTimeout(() => document.documentElement.removeAttribute("class"), 0);
}
</script>
<iframe onload="frameLoaded(this)" width="200" height="500" srcdoc='<!doctype html><img srcset="/images/green-256x256.png 100w" style="max-width: 100%" sizes="(min-width: 400px) 10px, 20px">'></iframe>
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
  "source_name": "html/semantics/embedded-content/the-img-element/sizes/sizes-dynamic-001.html"
}
```
