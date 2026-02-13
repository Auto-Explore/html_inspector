# html/semantics/embedded-content/the-img-element/sizes/sizes-auto-rendering-dynamic.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/sizes/sizes-auto-rendering-dynamic.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html class="reftest-wait">
<title>Auto sizes dynamic rendering</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/images.html#sizes-attributes">
<link rel="match" href="reference/sizes-auto-rendering-ref.html">
<script src="/common/rendering-utils.js"></script>
<script src="/common/reftest-wait.js"></script>
<img
  id="testImg"
  loading="lazy"
  sizes="auto"
  width="1"
  height="13"
>
<script>
  function secondImageLoaded() {
    waitForAtLeastOneFrame().then(takeScreenshot);
  }

  function firstImageLoaded() {
    const expected = 'red.png';
    if (!testImg.currentSrc.endsWith('red.png')) {
        const fail_msg = `FAIL: currentSrc is ${testImg.currentSrc}, expected ${expected}.`;
        document.body.textContent = fail_msg;
        takeScreenshot();
    }

    testImg.addEventListener('load', secondImageLoaded);
    testImg.style.width = '33px';
  }

  testImg.addEventListener('load', firstImageLoaded, {once: true});
  testImg.setAttribute('srcset', `
    /images/fail.gif 1000w,
    /images/green.png 100w,
    /images/red.png 10w
  `);
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.img.sizes.requires_srcset",
      "message": "The “sizes” attribute must only be specified if the “srcset” attribute is also specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 425,
        "byte_start": 346,
        "col": 1,
        "line": 8
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 425,
        "byte_start": 346,
        "col": 1,
        "line": 8
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 425,
        "byte_start": 346,
        "col": 1,
        "line": 8
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
  "source_name": "html/semantics/embedded-content/the-img-element/sizes/sizes-auto-rendering-dynamic.html"
}
```
