# html/semantics/embedded-content/the-img-element/image-loading-lazy-move-document.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/image-loading-lazy-move-document.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<head>
<title>Moving loading='lazy' image into another top level document</title>
<link rel="help" href="https://github.com/scott-little/lazyload">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
</head>

<div style="height:1000vh;"></div>
<img loading="lazy"
     src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABQAAAAUCAIAAAAC64paAAAAG0lEQVR42mP8z0A%2BYKJA76jmUc2jmkc1U0EzACKcASfOgGoMAAAAAElFTkSuQmCC">
<script>
promise_test(async t => {
  let image_loaded = false;
  const img = document.querySelector("img");
  img.addEventListener("load", () => { image_loaded = true; });

  await new Promise(resolve => window.addEventListener("load", resolve));

  assert_false(image_loaded,
               "lazy-load image shouldn't be loaded yet");

  const anotherWin = window.open("resources/newwindow.html");

  await new Promise(resolve => anotherWin.addEventListener("load", resolve));

  anotherWin.document.body.appendChild(img);

  assert_false(image_loaded,
               "lazy-load image shouldn't be loaded yet");

  img.scrollIntoView();

  await new Promise(resolve => img.addEventListener("load", resolve));
  assert_true(img.complete,
              "Now the lazy-load image should be loaded");
});
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
        "byte_end": 482,
        "byte_start": 314,
        "col": 1,
        "line": 10
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
  "source_name": "html/semantics/embedded-content/the-img-element/image-loading-lazy-move-document.html"
}
```
