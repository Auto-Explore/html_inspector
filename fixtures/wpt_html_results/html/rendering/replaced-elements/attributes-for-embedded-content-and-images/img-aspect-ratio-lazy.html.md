# html/rendering/replaced-elements/attributes-for-embedded-content-and-images/img-aspect-ratio-lazy.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/replaced-elements/attributes-for-embedded-content-and-images/img-aspect-ratio-lazy.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Image width and height attributes are used to infer aspect-ratio for lazy-loaded images</title>
<meta name="viewport" content="width=device-width">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<style>
  img {
    width: 100%;
    max-width: 100px;
    height: auto;
  }
</style>
<div style="height: 600vh"></div>
<img src="/images/green.png" loading="lazy" width=100 height=100>
<script>
let t = async_test("Image width and height attributes are used to infer aspect-ratio for lazy-loaded images");

function assert_ratio(img, expected) {
  let epsilon = 0.001;
  assert_approx_equals(parseFloat(getComputedStyle(img).width, 10) / parseFloat(getComputedStyle(img).height, 10), expected, epsilon);
}

t.step(function() {
  let img = document.querySelector("img");
  // The initial aspect ratio is given by the width/height attributes:
  // https://html.spec.whatwg.org/#map-to-the-aspect-ratio-property-(using-dimension-rules)
  assert_ratio(img, 1.0);
  img.addEventListener("load", t.step_func_done(function() {
    // Now the element "represents an image":
    // https://html.spec.whatwg.org/multipage/rendering.html#images-3
    // 2.0 is the original aspect ratio of green.png
    assert_ratio(img, 2.0);
  }));
  window.scrollTo(0, img.getBoundingClientRect().top);
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
        "byte_end": 462,
        "byte_start": 397,
        "col": 1,
        "line": 14
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
  "source_name": "html/rendering/replaced-elements/attributes-for-embedded-content-and-images/img-aspect-ratio-lazy.html"
}
```
