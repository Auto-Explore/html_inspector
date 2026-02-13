# html/semantics/embedded-content/the-img-element/image-loading-lazy-empty-src.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/image-loading-lazy-empty-src.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<title>Lazy loaded Images handle correctly when setting src to empty</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/images.html#update-the-image-data">
<div id=log></div>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<img id="image" loading="lazy" src="resources/image.png">

<script>
  const image = document.querySelector('#image');

async_test(function(t) {
    image.onerror = t.step_func(function(e) {
        assert_equals(e.type, "error", "null image source check failed");
        image.onload = t.step_func(function() {
            t.done();
        });
        image.src = "resources/image.png";
    });
    image.src = "";
}, "lazy loaded image and empty src");

</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 381,
        "byte_start": 324,
        "col": 1,
        "line": 9
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/embedded-content/the-img-element/image-loading-lazy-empty-src.html"
}
```
