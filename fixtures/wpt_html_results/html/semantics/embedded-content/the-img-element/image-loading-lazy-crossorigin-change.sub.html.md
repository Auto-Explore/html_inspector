# html/semantics/embedded-content/the-img-element/image-loading-lazy-crossorigin-change.sub.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/image-loading-lazy-crossorigin-change.sub.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<head>
  <title>Deferred images with loading='lazy' use the latest crossorigin attribute</title>
  <link rel="author" title="Raj T" href="mailto:rajendrant@chromium.org">
  <link rel="author" title="Rob Buis" href="mailto:rbuis@igalia.com">
  <script src="/resources/testharness.js"></script>
  <script src="/resources/testharnessreport.js"></script>
  <script src="../resources/common.js"></script>
</head>

<script>
  const img = new ElementLoadPromise("cross-origin");

  async_test(function(t) {
    window.addEventListener("load", t.step_func(() => {
      // At this point the image's #updating-the-image-data algorithm has been
      // invoked, and the image request has been deferred. The deferred
      // cross-origin image request was created with the `no-cors` request mode,
      // which would succeed to load the cross-origin image.
      // While the request is deferred, we'll set the `crossorigin` attribute to a
      // value that would cause the image request to fail. Since `crossorigin`
      // mutations trigger another #updating-the-image-data invocation (replacing
      // the first one), when we scroll the image into view, the image should be
      // fetched with the latest `crossorigin` attribute value, and fail to load.
      img.element().crossOrigin = 'anonymous';
      img.element().scrollIntoView();
    }));

    img.promise
      .then(t.unreached_func("The image should not load."))
      .catch(t.step_func(() => { img.element().onload = t.step_func_done(); img.element().src = 'resources/image.png'; }));
  }, "Test that when deferred image is loaded, it uses the latest crossorigin attribute.");
</script>

<body>
  <div style="height:1000vh;"></div>
  <img id="cross-origin" loading="lazy"
       src='http://{{hosts[alt][]}}:{{ports[http][0]}}/html/semantics/embedded-content/the-img-element/resources/image.png'
       onload="img.resolve();" onerror="img.reject();">
</body>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.img.src.invalid",
      "message": "Bad value “http://{{hosts[alt][]}}:{{ports[http][0]}}/html/semantics/embedded-content/the-img-element/resources/image.png” for attribute “src” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1933,
        "byte_start": 1716,
        "col": 3,
        "line": 37
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 1933,
        "byte_start": 1716,
        "col": 3,
        "line": 37
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
  "source_name": "html/semantics/embedded-content/the-img-element/image-loading-lazy-crossorigin-change.sub.html"
}
```
