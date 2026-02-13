# html/semantics/embedded-content/the-img-element/naturalWidth-naturalHeight-width-height.tentative.html

Counts:
- errors: 0
- warnings: 74
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/naturalWidth-naturalHeight-width-height.tentative.html",
  "validated_html_truncated": true,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<head>
<meta charset="utf-8">
<title>HTMLImageElement attributes naturalWidth, naturalHeight, width, height</title>
<!-- Note: this test asserts a different expectation from what the HTML spec
     requires, as of mid-2025 when this testcase is being written. The spec
     behavior doesn't appear to be web-compatible for some of the cases here,
     and issue https://github.com/whatwg/html/issues/11287 is filed on
     addresing that.  In the meantime, this test is named with ".tentative" to
     indicate that it's not authoritative. After the spec change is accepted,
     we can remove the neighboring naturalWidth-naturalHeight.html test which
     asserts the prior spec text's expectations, since this test covers the
     same ground but with its expectations set according to the
     soon-to-be-updated spec text.  -->
<link rel="help" href="https://github.com/whatwg/html/issues/11287">
<link rel="help" href="https://html.spec.whatwg.org/multipage/embedded-content.html#dom-img-naturalwidth-dev">
<link rel="help" href="https://html.spec.whatwg.org/multipage/embedded-content.html#dom-img-width">
<link rel="help" href="https://html.spec.whatwg.org/multipage/images.html#density-corrected-intrinsic-width-and-height">
<link rel="help" href="https://drafts.csswg.org/css-images/#natural-dimensions">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<style>
#scroller {
  /* We wrap all the test content in a scroller so that it doesn't push
   * the textual test-results too far out of view.
   */
  border: 1px solid black;
  height: 400px;
  width: max-content;
  overflow: scroll;
}
#containingBlock {
  /* There are a few SVG images here that size so that their margin-box fills
   * their containing block width. We define a specific size here so that we
   * can then check for it (minus the margins) in the "data-width" attribute.
   */
  width: 740px;
}
img {
  /* This styling is just cosmetic, to help visualize the images. */
  border: 5px solid teal;
  margin: 5px;
  display: block;
}
</style>
<!-- We specify the img elements in a <template> and then clone them for
     testing, so that we can dynamically generate and test several variants
     of each img. -->
<template id="imgTemplates">
<!-- For each img element:
     * The "data-natural-{width,height}" attributes represent the expected
     values of the img element's "naturalWidth" and "naturalHeight" IDL
     attributes. This test implicitly expects the "width" and "height" IDL
     attributes to have those same expected values; but in cases where that's
     not correct, we provide the actual expected value in the
     "data-{width,height}" attributes (as distinguished from
     data-natural-{width,height}).
     * The "title" attribute is a description of the scenario being tested, and
     it must be unique to satisfy the test harness requirements. -->

<!-- JPG images -->
<img src="resources/cat.jpg"
     title="raster image"
     data-natural-width="320" data-natural-height="240">
<img src="resources/cat.jpg" width="10" height="10"
     title="raster image with width/height attributes"
     data-natural-width="320" data-natural-height="240"
     data-width="10" data-height="10"
     data-not-rendered-width="10" data-not-rendered-height="10">
<!-- Use "size-comes-from-broken-image-icon" attribute to opt out of checking
     img.width and img.height, because they come from the UA-dependent
     size of the broken image icon: -->
<img src="non-existent.jpg"
     title="non existent image, no natural dimensions"
     data-natural-width="0" data-natural-height="0"
     size-comes-from-broken-image-icon>
<img src="non-existent.jpg" width="10" height="10"
     title="non existent image with width/height attributes, no natural dimensions"
     data-natural-width="0" data-natural-height="0"
     data-width="10" data-height="10"
     data-not-rendered-width="10" data-not-rendered-height="10">

<!-- First group of SVG images: no viewBox, with a missing (or edge-casey, i.e.
     negative or percent-valued) value for the width and/or height attr on the
     root svg element in a SVG image. -->
<img src="data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg'></svg>"
     title="SVG image, no natural dimensions"
     data-natural-width="300" data-natural-height="150">
<img src="data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg'></svg>"
     width="40" height="10"
     data-width="40" data-height="10"
     data-not-rendered-width="40" data-not-rendered-height="10"
     title="SVG image with width/height attrs, no natural dimensions"
     data-natural-width="300" data-natural-height="150">
<img src="data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg'></svg>"
     width="40"
     data-width="40" data-not-rendered-width="40"
     title="SVG image with width attr, no natural dimensions"
     data-natural-width="300" data-natural-height="150">
<img src="data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg'></svg>"
     height="10"
     data-height="10" data-not-rendered-height="10"
     title="SVG image with height attr, no natural dimensions"
     data-natural-width="300" data-natural-height="150">
<!-- Note: percent values can't be resolved when determining natural
     dimensions, so the exact percentage shouldn't matter. -->
<img src="data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' width='400%' height='10%'></svg>"
     title="SVG image, percengage natural dimensions"
     data-natural-width="300" data-natural-height="150">
<img src="data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' width='-400%' height='-10%'></svg>"
     title="SVG image, negative percengage natural dimensions"
     data-natural-width="300" data-natural-height="150">
<!-- If only one attribute is present, it should show up as a natural
     dimension, without influencing the other natural dimension. -->
<img src="data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' width='60'></svg>"
     title="SVG image, with natural width"
     data-natural-width="60" data-natural-height="150">
<img src="data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' height='60'></svg>"
     title="SVG image, with natural height"
     data-natural-width="300" data-natural-height="60">
<!-- If either attribute is 0 or a negative length, it should show up as a
     natural dimension: of 0. -->
<img src="data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' width='0'></svg>"
     title="SVG image, with natural width of 0"
     data-natural-width="0" data-natural-height="150">
<img src="data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' height='0'></svg>"
     title="SVG image, with natural height of 0"
     data-natural-width="300" data-natural-height="0">
<img src="data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' width='-5'></svg>"
     title="SVG image, with natural width being negative"
     data-natural-width="300" data-natural-height="150">
<img src="data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' height='-5'></svg>"
     title="SVG image, with natural height being negative"
     data-natural-width="300" data-natural-height="150">

<!-- Second group of SVG images: Same as above, but now with a viewBox that grants a
     3:1 aspect-ratio; whenever we know one natural dimension, that should
     combine with the aspect ratio to produce the other natural dimension.

     NOTE: for a few subtests here, the image ends up expanding to fill the
     containing block's width, i.e. rendering at a larger size than its natural
     size. In those cases, we include 'data-width' & 'data-height' attributes,
     so that this test's JS can validate that img.width and img.height return
     these expected larger values. (Otherwise, we expect img.width and
     img.height to return the same values as img.naturalWidth and
     img.naturalHeight). -->
<img src="data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 600 200'></svg>"
     title="SVG image, no natural dimensions, and aspect ratio from viewBox"
     data-natural-width="300" data-natural-height="100"
     data-width="720" data-height="240">
<img src="data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' width='400%' height='10%' viewBox='0 0 600 200'></svg>"
     title="SVG image, percengage natural dimensions, and aspect ratio from viewBox"
     data-natural-width="300" data-natural-height="100"
     data-width="720" data-height="240">
<img src="data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' width='-400%' height='-10%' viewBox='0 0 600 200'></svg>"
     title="SVG image, negative percengage natural dimensions, and aspect ratio from viewBox"
     data-natural-width="300" data-natural-height="100"
     data-width="720" data-height="240">
<img src="data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' width='60' viewBox='0 0 600 200'></svg>"
     title="SVG image, with natural width, and aspect ratio from viewBox"
     data-natural-width="60" data-natural-height="20">
<img src="data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' height='60' viewBox='0 0 600 200'></svg>"
     title="SVG image, with natural height, and aspect ratio from viewBox"
     data-natural-width="180" data-natural-height="60">
<img src="data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' width='0' viewBox='0 0 600 200'></svg>"
     title="SVG image, with natural width of 0, and aspect ratio from viewBox"
     data-natural-width="0" data-natural-height="0">
<img src="data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' height='0' viewBox='0 0 600 200'></svg>"
     title="SVG image, with natural height of 0, and aspect ratio from viewBox"
     data-natural-width="0" data-natural-height="0">
<img src="data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' width='-5' viewBox='0 0 600 200'></svg>"
     title="SVG image, with natural width being negative, and aspect ratio from viewBox"
     data-natural-width="300" data-natural-height="100"
     data-width="720" data-height="240">
<img src="data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' height='-5' viewBox='0 0 600 200'></svg>"
     title="SVG image, with natural height being negative, and aspect ratio from viewBox"
     data-natural-width="300" data-natural-height="100"
     data-width="720" data-height="240">

<!-- Third group of SVG images: Check a degenerate 0-sized viewBox for some of the
     cases; it should have no impact. -->
<img src="data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 0 0'></svg>"
     title="SVG image, no natural dimensions, viewBox with 0 width/height"
     data-natural-width="300" data-natural-height="150">
<img src="data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 10 0'></svg>"
     title="SVG image, no natural dimensions, viewBox with 0 width"
     data-natural-width="300" data-natural-height="150">
<img src="data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 0 10'></svg>"
     title="SVG image, no natural dimensions, viewBox with 0 height"
     data-natural-width="300" data-natural-height="150">
<img src="data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' width='60' viewBox='0 0 0 0'></svg>"
     title="SVG image, with natural width, viewBox with 0 width/height"
     data-natural-width="60" data-natural-height="150">
<img src="data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' width='60' viewBox='0 0 10 0'></svg>"
     title="SVG image, with natural width, viewBox with 0 width"
     data-natural-width="60" data-natural-height="150">
<img src="data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' width='60' viewBox='0 0 0 10'></svg>"
     title="SVG image, with natural width, viewBox with 0 height"
     data-natural-width="60" data-natural-height="150">
<img src="data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' height='60' viewBox='0 0 0 0'></svg>"
     title="SVG image, with natural height, viewBox with 0 width/height"
     data-natural-width="300" data-natural-height="60">
<img src="data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' height='60' viewBox='0 0 10 0'></svg>"
     title="SVG image, with natural height, viewBox with 0 width"
     data-natural-width="300" data-natural-height="60">
<img src="data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' height='60' viewBox='0 0 0 10'></svg>"
     title="SVG image, with natural height, viewBox with 0 height"
     data-natural-width="300" data-natural-height="60">

<!~- Final group of SVG images: we have pixel-valued width/height on the root
     svg element, and possibly viewBox as well. The width and height attrs should
     determine the natural dimensions, with no impact from viewBox. -->
<img src="data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' width='60' height='40'></svg>"
     title="SVG image, with natural width and height"
     data-natural-width="60" data-natural-height="40">
<img src="data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' width='60' height='40' viewBox='0 0 600 200'></svg>"
     title="SVG image, with natural width and height, and aspect ratio from viewBox"
     data-natural-width="60" data-natural-height="40">
<img src="data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' width='0' height='0' viewBox='0 0 600 200'></svg>"
     title="SVG image, with natural width and height of 0, and aspect ratio from viewBox"
     data-natural-width="0" data-natural-height="0">
<img src="data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' width='-5' height='-5' viewBox='0 0 600 200'></svg>"
     title="SVG image, with natural width and height being negative, and aspect ratio from viewBox"
     data-natural-width="300" data-natural-height="100"
     data-width="720" data-height="240">
</template>
</head>
<body>
<div id="scroller">
  <div id="containingBlock">
  </div>
</div>
<!-- We generate and append all of the tested <img> elements while we're inside
     the <body>, so that all of the <img> elements' "load" events will block
     the window onload event: -->
<script>
setup({explicit_done:true});

// Utility function to generate a clone of imgTemplates using "srcset" and a
// given density value, with expectations adjusted per the density:
function cloneTemplateWithDensity(density) {
  if (typeof(density) !== "number" || density <= 0) {
    // If we get here, there's a bug in the test itself; throw an exception:
    throw new Error("test error: param should be a positive number");
  }
  let clone = imgTemplates.content.cloneNode("true");

  for (let img of clone.children) {
    // Swap out "src" for "srcset":
    // The srcset attribute uses a space-separated list of tokens,
    // so we need to encode any spaces that are in the URI itself
    // before we put it in srcset:
    let srcVal = img.getAttribute("src").replaceAll(" ", "%20");
    img.removeAttribute("src");
    img.setAttribute("srcSet", `${srcVal} ${density}x`);

    // Mention the density in the 'title' to keep the title values unique:
    img.setAttribute("title",
                     `${img.getAttribute("title")} (with srcset/${density}x)`);

    const isUsingConcreteObjectWidth = (img.dataset.naturalWidth == "300");
    const isUsingConcreteObjectHeight = (img.dataset.naturalHeight == "150");

    // Scale our 'data-natural-{width,height}' expectations by the density.
    // But also:
    // * Preserve the original 'data-natural-{width,height}' as the
    // 'data-{width,height}' expectation if it's just the concrete object size
    // (which doesn't actually get scaled by the density in practice when
    // the image actually renders).
    // * Preserve the original 'data-natural-{width,height}' as the
    // 'data-not-rendered-{width,height}' expectation (if we don't already have
    // one) since browsers don't do density-correction on .width and .height when
    // the image is not being rendered, as discussed in
    // https://github.com/whatwg/html/issues/11287#issuecomment-2923467541
    for (let name in img.dataset) {
      if (name.startsWith("natural")) {
        let origExpectation = img.dataset[name];

        // Scale our img.natural{Width,Height} expectation:
        img.dataset[name] = origExpectation / density;

        let isWidthAxis = (name == "naturalWidth");
        let nameW
```
(validated HTML truncated to first 16384 bytes)

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 3061,
        "byte_start": 2950,
        "col": 1,
        "line": 62
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 3327,
        "byte_start": 3062,
        "col": 1,
        "line": 65
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 3691,
        "byte_start": 3517,
        "col": 1,
        "line": 73
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 3981,
        "byte_start": 3692,
        "col": 1,
        "line": 77
      }
    },
    {
      "category": "Html",
      "code": "html.img.src.invalid",
      "message": "Bad value “data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg'></svg>” for attribute “src” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 4363,
        "byte_start": 4184,
        "col": 1,
        "line": 86
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 4363,
        "byte_start": 4184,
        "col": 1,
        "line": 86
      }
    },
    {
      "category": "Html",
      "code": "html.img.src.invalid",
      "message": "Bad value “data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg'></svg>” for attribute “src” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 4697,
        "byte_start": 4364,
        "col": 1,
        "line": 89
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 4697,
        "byte_start": 4364,
        "col": 1,
        "line": 89
      }
    },
    {
      "category": "Html",
      "code": "html.img.src.invalid",
      "message": "Bad value “data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg'></svg>” for attribute “src” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 4959,
        "byte_start": 4698,
        "col": 1,
        "line": 95
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 4959,
        "byte_start": 4698,
        "col": 1,
        "line": 95
      }
    },
    {
      "category": "Html",
      "code": "html.img.src.invalid",
      "message": "Bad value “data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg'></svg>” for attribute “src” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 5225,
        "byte_start": 4960,
        "col": 1,
        "line": 100
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 5225,
        "byte_start": 4960,
        "col": 1,
        "line": 100
      }
    },
    {
      "category": "Html",
      "code": "html.img.src.invalid",
      "message": "Bad value “data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' width='400%' height='10%'></svg>” for attribute “src” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 5571,
        "byte_start": 5358,
        "col": 1,
        "line": 107
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 5571,
        "byte_start": 5358,
        "col": 1,
        "line": 107
      }
    },
    {
      "category": "Html",
      "code": "html.img.src.invalid",
      "message": "Bad value “data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' width='-400%' height='-10%'></svg>” for attribute “src” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 5796,
        "byte_start": 5572,
        "col": 1,
        "line": 110
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 5796,
        "byte_start": 5572,
        "col": 1,
        "line": 110
      }
    },
    {
      "category": "Html",
      "code": "html.img.src.invalid",
      "message": "Bad value “data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' width='60'></svg>” for attribute “src” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 6122,
        "byte_start": 5936,
        "col": 1,
        "line": 115
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 6122,
        "byte_start": 5936,
        "col": 1,
        "line": 115
      }
    },
    {
      "category": "Html",
      "code": "html.img.src.invalid",
      "message": "Bad value “data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' height='60'></svg>” for attribute “src” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 6311,
        "byte_start": 6123,
        "col": 1,
        "line": 118
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 6311,
        "byte_start": 6123,
        "col": 1,
        "line": 118
      }
    },
    {
      "category": "Html",
      "code": "html.img.src.invalid",
      "message": "Bad value “data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' width='0'></svg>” for attribute “src” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 6610,
        "byte_start": 6421,
        "col": 1,
        "line": 123
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 6610,
        "byte_start": 6421,
        "col": 1,
        "line": 123
      }
    },
    {
      "category": "Html",
      "code": "html.img.src.invalid",
      "message": "Bad value “data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' height='0'></svg>” for attribute “src” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 6802,
        "byte_start": 6611,
        "col": 1,
        "line": 126
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 6802,
        "byte_start": 6611,
        "col": 1,
        "line": 126
      }
    },
    {
      "category": "Html",
      "code": "html.img.src.invalid",
      "message": "Bad value “data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' width='-5'></svg>” for attribute “src” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 7005,
        "byte_start": 6803,
        "col": 1,
        "line": 129
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 7005,
        "byte_start": 6803,
        "col": 1,
        "line": 129
      }
    },
    {
      "category": "Html",
      "code": "html.img.src.invalid",
      "message": "Bad value “data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' height='-5'></svg>” for attribute “src” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 7210,
        "byte_start": 7006,
        "col": 1,
        "line": 132
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 7210,
        "byte_start": 7006,
        "col": 1,
        "line": 132
      }
    },
    {
      "category": "Html",
      "code": "html.img.src.invalid",
      "message": "Bad value “data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 600 200'></svg>” for attribute “src” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 8199,
        "byte_start": 7927,
        "col": 1,
        "line": 147
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 8199,
        "byte_start": 7927,
        "col": 1,
        "line": 147
      }
    },
    {
      "category": "Html",
      "code": "html.img.src.invalid",
      "message": "Bad value “data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' width='400%' height='10%' viewBox='0 0 600 200'></svg>” for attribute “src” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 8506,
        "byte_start": 8200,
        "col": 1,
        "line": 151
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 8506,
        "byte_start": 8200,
        "col": 1,
        "line": 151
      }
    },
    {
      "category": "Html",
      "code": "html.img.src.invalid",
      "message": "Bad value “data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' width='-400%' height='-10%' viewBox='0 0 600 200'></svg>” for attribute “src” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 8824,
        "byte_start": 8507,
        "col": 1,
        "line": 155
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 8824,
        "byte_start": 8507,
        "col": 1,
        "line": 155
      }
    },
    {
      "category": "Html",
      "code": "html.img.src.invalid",
      "message": "Bad value “data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' width='60' viewBox='0 0 600 200'></svg>” for attribute “src” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 9063,
        "byte_start": 8825,
        "col": 1,
        "line": 159
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 9063,
        "byte_start": 8825,
        "col": 1,
        "line": 159
      }
    },
    {
      "category": "Html",
      "code": "html.img.src.invalid",
      "message": "Bad value “data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' height='60' viewBox='0 0 600 200'></svg>” for attribute “src” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 9305,
        "byte_start": 9064,
        "col": 1,
        "line": 162
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 9305,
        "byte_start": 9064,
        "col": 1,
        "line": 162
      }
    },
    {
      "category": "Html",
      "code": "html.img.src.invalid",
      "message": "Bad value “data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' width='0' viewBox='0 0 600 200'></svg>” for attribute “src” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 9546,
        "byte_start": 9306,
        "col": 1,
        "line": 165
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 9546,
        "byte_start": 9306,
        "col": 1,
        "line": 165
      }
    },
    {
      "category": "Html",
      "code": "html.img.src.invalid",
      "message": "Bad value “data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' height='0' viewBox='0 0 600 200'></svg>” for attribute “src” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 9789,
        "byte_start": 9547,
        "col": 1,
        "line": 168
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 9789,
        "byte_start": 9547,
        "col": 1,
        "line": 168
      }
    },
    {
      "category": "Html",
      "code": "html.img.src.invalid",
      "message": "Bad value “data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' width='-5' viewBox='0 0 600 200'></svg>” for attribute “src” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 10085,
        "byte_start": 9790,
        "col": 1,
        "line": 171
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 10085,
        "byte_start": 9790,
        "col": 1,
        "line": 171
      }
    },
    {
      "category": "Html",
      "code": "html.img.src.invalid",
      "message": "Bad value “data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' height='-5' viewBox='0 0 600 200'></svg>” for attribute “src” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 10383,
        "byte_start": 10086,
        "col": 1,
        "line": 175
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 10383,
        "byte_start": 10086,
        "col": 1,
        "line": 175
      }
    },
    {
      "category": "Html",
      "code": "html.img.src.invalid",
      "message": "Bad value “data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 0 0'></svg>” for attribute “src” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 10736,
        "byte_start": 10510,
        "col": 1,
        "line": 182
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 10736,
        "byte_start": 10510,
        "col": 1,
        "line": 182
      }
    },
    {
      "category": "Html",
      "code": "html.img.src.invalid",
      "message": "Bad value “data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 10 0'></svg>” for attribute “src” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 10957,
        "byte_start": 10737,
        "col": 1,
        "line": 185
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 10957,
        "byte_start": 10737,
        "col": 1,
        "line": 185
      }
    },
    {
      "category": "Html",
      "code": "html.img.src.invalid",
      "message": "Bad value “data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 0 10'></svg>” for attribute “src” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 11179,
        "byte_start": 10958,
        "col": 1,
        "line": 188
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 11179,
        "byte_start": 10958,
        "col": 1,
        "line": 188
      }
    },
    {
      "category": "Html",
      "code": "html.img.src.invalid",
      "message": "Bad value “data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' width='60' viewBox='0 0 0 0'></svg>” for attribute “src” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 11413,
        "byte_start": 11180,
        "col": 1,
        "line": 191
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 11413,
        "byte_start": 11180,
        "col": 1,
        "line": 191
      }
    },
    {
      "category": "Html",
      "code": "html.img.src.invalid",
      "message": "Bad value “data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' width='60' viewBox='0 0 10 0'></svg>” for attribute “src” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 11641,
        "byte_start": 11414,
        "col": 1,
        "line": 194
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 11641,
        "byte_start": 11414,
        "col": 1,
        "line": 194
      }
    },
    {
      "category": "Html",
      "code": "html.img.src.invalid",
      "message": "Bad value “data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' width='60' viewBox='0 0 0 10'></svg>” for attribute “src” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 11870,
        "byte_start": 11642,
        "col": 1,
        "line": 197
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 11870,
        "byte_start": 11642,
        "col": 1,
        "line": 197
      }
    },
    {
      "category": "Html",
      "code": "html.img.src.invalid",
      "message": "Bad value “data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' height='60' viewBox='0 0 0 0'></svg>” for attribute “src” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 12106,
        "byte_start": 11871,
        "col": 1,
        "line": 200
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 12106,
        "byte_start": 11871,
        "col": 1,
        "line": 200
      }
    },
    {
      "category": "Html",
      "code": "html.img.src.invalid",
      "message": "Bad value “data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' height='60' viewBox='0 0 10 0'></svg>” for attribute “src” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 12336,
        "byte_start": 12107,
        "col": 1,
        "line": 203
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 12336,
        "byte_start": 12107,
        "col": 1,
        "line": 203
      }
    },
    {
      "category": "Html",
      "code": "html.img.src.invalid",
      "message": "Bad value “data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' height='60' viewBox='0 0 0 10'></svg>” for attribute “src” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 12567,
        "byte_start": 12337,
        "col": 1,
        "line": 206
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 12567,
        "byte_start": 12337,
        "col": 1,
        "line": 206
      }
    },
    {
      "category": "Html",
      "code": "html.tokenizer.bogus_comment",
      "message": "Bogus comment.",
      "severity": "Warning",
      "span": {
        "byte_end": 12571,
        "byte_start": 12569,
        "col": 1,
        "line": 210
      }
    },
    {
      "category": "Html",
      "code": "html.img.src.invalid",
      "message": "Bad value “data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' width='60' height='40'></svg>” for attribute “src” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 13009,
        "byte_start": 12801,
        "col": 1,
        "line": 213
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 13009,
        "byte_start": 12801,
        "col": 1,
        "line": 213
      }
    },
    {
      "category": "Html",
      "code": "html.img.src.invalid",
      "message": "Bad value “data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' width='60' height='40' viewBox='0 0 600 200'></svg>” for attribute “src” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 13271,
        "byte_start": 13010,
        "col": 1,
        "line": 216
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 13271,
        "byte_start": 13010,
        "col": 1,
        "line": 216
      }
    },
    {
      "category": "Html",
      "code": "html.img.src.invalid",
      "message": "Bad value “data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' width='0' height='0' viewBox='0 0 600 200'></svg>” for attribute “src” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 13534,
        "byte_start": 13272,
        "col": 1,
        "line": 219
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 13534,
        "byte_start": 13272,
        "col": 1,
        "line": 219
      }
    },
    {
      "category": "Html",
      "code": "html.img.src.invalid",
      "message": "Bad value “data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' width='-5' height='-5' viewBox='0 0 600 200'></svg>” for attribute “src” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 13853,
        "byte_start": 13535,
        "col": 1,
        "line": 222
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 13853,
        "byte_start": 13535,
        "col": 1,
        "line": 222
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
  "source_name": "html/semantics/embedded-content/the-img-element/naturalWidth-naturalHeight-width-height.tentative.html"
}
```
