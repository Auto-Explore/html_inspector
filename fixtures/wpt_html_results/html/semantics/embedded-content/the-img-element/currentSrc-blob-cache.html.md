# html/semantics/embedded-content/the-img-element/currentSrc-blob-cache.html

Counts:
- errors: 0
- warnings: 5
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/currentSrc-blob-cache.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<title>currentSrc is right even if underlying image is a shared blob</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<link rel="help" href="https://bugzilla.mozilla.org/show_bug.cgi?id=1625786">
<link rel="author" title="Emilio Cobos Álvarez" href="mailto:emilio@crisal.io">
<link rel="author" title="Mozilla" href="https://mozilla.org">
<img id="first">
<img id="second">
<script>
promise_test(async t => {
  let canvas = document.createElement("canvas");
  canvas.width = 100;
  canvas.height = 100;
  let ctx = canvas.getContext("2d");
  ctx.fillStyle = "green";
  ctx.rect(0, 0, 100, 100);
  ctx.fill();

  let blob = await new Promise(resolve => canvas.toBlob(resolve));

  let first = document.querySelector("#first");
  let second = document.querySelector("#second");

  let firstLoad = new Promise(resolve => {
    first.addEventListener("load", resolve, { once: true });
  });

  let secondLoad = new Promise(resolve => {
    second.addEventListener("load", resolve, { once: true });
  });

  let uri1 = URL.createObjectURL(blob);
  let uri2 = URL.createObjectURL(blob);
  first.src = uri1;
  second.src = uri2;

  await firstLoad;
  await secondLoad;

  assert_equals(first.src, first.currentSrc);
  assert_equals(second.src, second.currentSrc);
});
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 460,
        "byte_start": 444,
        "col": 1,
        "line": 9
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 460,
        "byte_start": 444,
        "col": 1,
        "line": 9
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 478,
        "byte_start": 461,
        "col": 1,
        "line": 10
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 478,
        "byte_start": 461,
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
  "source_name": "html/semantics/embedded-content/the-img-element/currentSrc-blob-cache.html"
}
```
