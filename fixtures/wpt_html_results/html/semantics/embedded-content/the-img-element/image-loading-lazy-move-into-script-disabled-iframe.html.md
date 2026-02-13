# html/semantics/embedded-content/the-img-element/image-loading-lazy-move-into-script-disabled-iframe.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/image-loading-lazy-move-into-script-disabled-iframe.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<head>
<title>A loading='lazy' image starts loading when the element is moved into
       an iframe where script is disabled</title>
<link rel="help" href="https://github.com/scott-little/lazyload">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
</head>

<div style="height:1000vh;"></div>
<iframe id="iframe" src="resources/image-loading-lazy-in-viewport.html">
</iframe>
<iframe id="sandboxediframe" sandbox="allow-same-origin">
</iframe>
<script>
promise_test(async t => {
  await new Promise(resolve => window.addEventListener('load', resolve));

  const image = iframe.contentDocument.querySelector("img");

  assert_false(image.complete, "lazy-load image shouldn't be loaded");

  sandboxediframe.contentDocument.body.appendChild(image);
  await new Promise(resolve => image.addEventListener("load", resolve));

  assert_true(image.complete,
              "lazy-load image shouldn't be honored in script disabled iframe");
});
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
  "source_name": "html/semantics/embedded-content/the-img-element/image-loading-lazy-move-into-script-disabled-iframe.html"
}
```
