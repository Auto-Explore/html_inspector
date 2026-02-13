# html/semantics/embedded-content/the-img-element/picture-loading-lazy.html

Counts:
- errors: 0
- warnings: 7
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/picture-loading-lazy.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<head>
  <title>Images with loading='lazy' in picture elements load when near the viewport</title>
  <link rel="author" title="Raj T" href="mailto:rajendrant@chromium.org">
  <link rel="help" href="https://github.com/scott-little/lazyload">
  <script src="/resources/testharness.js"></script>
  <script src="/resources/testharnessreport.js"></script>
  <script src="../resources/common.js"></script>
</head>

<script>
const in_viewport_img = new ElementLoadPromise("in_viewport_img");
const lazy_attribute_img = new ElementLoadPromise("lazy_attribute_img");
const eager_attribute_img = new ElementLoadPromise("eager_attribute_img");

const document_load_promise = new Promise(resolve => {
  window.addEventListener("load", resolve);
});

async_test(function(t) {
  document_load_promise.then(t.step_func_done(function() {
    assert_false(lazy_attribute_img.element().complete);
    lazy_attribute_img.element().scrollIntoView();
  }));
}, "Test that the loading=lazy <picture> element below viewport was deferred, on document load.");

async_test(function(t) {
  in_viewport_img.promise.then(t.step_func_done());
}, "Test that in viewport <picture> element was loaded");

async_test(function(t) {
  eager_attribute_img.promise.then(t.step_func_done());
}, "Test that eager <picture> element was loaded");

async_test(function(t) {
  lazy_attribute_img.promise.then(t.step_func_done());
}, "Test that deferred <picture> element was loaded-in as well, after scrolled down");

</script>

<body>
<picture>
  <source sizes='50vw' srcset='resources/image.png?in_viewport_img'>
  <img id='in_viewport_img' src='img-not-loaded.png' loading="lazy" onload="in_viewport_img.resolve();">
</picture>
<div style="height:10000px;"></div>
<picture>
  <source sizes='50vw' srcset='resources/image.png?lazy_attribute_img'>
  <img id='lazy_attribute_img' src='img-not-loaded.png' loading="lazy" onload="lazy_attribute_img.resolve();">
</picture>
<picture>
  <source sizes='50vw' srcset='resources/image.png?eager_attribute_img'>
  <img id='eager_attribute_img' src='img-not-loaded.png' loading="eager" onload="eager_attribute_img.resolve();">
</picture>

<!--
  This async script loads very slowly in order to ensure that, if the
  below_viewport image has started loading, it has a chance to finish
  loading before window.load() happens, so that the test will dependably fail
  in that case instead of potentially passing depending on how long different
  resource fetches take.
-->
<script async src="/common/slow.py"></script>
</body>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.source.srcset.invalid",
      "message": "Bad value “resources/image.png?in_viewport_img” for attribute “srcset” on element “source”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1587,
        "byte_start": 1521,
        "col": 3,
        "line": 43
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 1692,
        "byte_start": 1590,
        "col": 3,
        "line": 44
      }
    },
    {
      "category": "Html",
      "code": "html.source.srcset.invalid",
      "message": "Bad value “resources/image.png?lazy_attribute_img” for attribute “srcset” on element “source”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1821,
        "byte_start": 1752,
        "col": 3,
        "line": 48
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 1932,
        "byte_start": 1824,
        "col": 3,
        "line": 49
      }
    },
    {
      "category": "Html",
      "code": "html.source.srcset.invalid",
      "message": "Bad value “resources/image.png?eager_attribute_img” for attribute “srcset” on element “source”.",
      "severity": "Warning",
      "span": {
        "byte_end": 2026,
        "byte_start": 1956,
        "col": 3,
        "line": 52
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 2140,
        "byte_start": 2029,
        "col": 3,
        "line": 53
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
  "source_name": "html/semantics/embedded-content/the-img-element/picture-loading-lazy.html"
}
```
