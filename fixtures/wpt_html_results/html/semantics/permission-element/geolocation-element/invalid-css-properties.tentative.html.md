# html/semantics/permission-element/geolocation-element/invalid-css-properties.tentative.html

Counts:
- errors: 0
- warnings: 5
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/permission-element/geolocation-element/invalid-css-properties.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<link rel="help" href="https://github.com/WICG/PEPC/blob/main/explainer.md#locking-the-pepc-style">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<body>
<!--The geolocation element does not allow certain CSS properties
-->
<style>
  #id1 {
    border-image: url('test-url');
    background-image: url('test-url');
    clip-path: circle(10px);
    filter: blur(10px);
    mask: url('test-url');
    padding-left: 10px; /* this is not allowed because width is not set to 'auto' */
    width: 200px;
    transform: rotate(10);
    border: 0px;
    cursor: none;
    content-visibility: hidden;
    contain: size;
    corner-shape: notch;
    corner-top-left-shape: scoop;
    corner-top-right-shape: bevel;
    corner-bottom-left-shape: squircle;
    corner-bottom-right-shape: square;
  }
</style>

<geolocation id="id1"></geolocation>

<script>
  test(function(){
    var el_with_negatives = document.getElementById("id1");
    assert_equals(getComputedStyle(el_with_negatives).borderImage, "none", "border-image");
    assert_equals(getComputedStyle(el_with_negatives).backgroundImage, "none", "background-image");
    assert_equals(getComputedStyle(el_with_negatives).clipPath, "none", "clip-path");
    assert_equals(getComputedStyle(el_with_negatives).filter, "none", "filter");
    assert_equals(getComputedStyle(el_with_negatives).mask, "none", "mask");
    assert_equals(getComputedStyle(el_with_negatives).paddingLeft, "0px", "padding-left");
    assert_equals(getComputedStyle(el_with_negatives).transform, "none", "transform");
    assert_equals(getComputedStyle(el_with_negatives).cursor, "pointer", "cursor");
    assert_equals(getComputedStyle(el_with_negatives).contentVisibility, "visible", "content-visibility");
    assert_equals(getComputedStyle(el_with_negatives).contain, "none", "contain");
    assert_equals(getComputedStyle(el_with_negatives).cornerShape, "round", "corner-shape");
    assert_equals(getComputedStyle(el_with_negatives).cornerTopLeftShape, "round", "corner-top-left-shape");
    assert_equals(getComputedStyle(el_with_negatives).cornerTopRightShape, "round", "corner-top-right-shape");
    assert_equals(getComputedStyle(el_with_negatives).cornerBottomLeftShape, "round", "corner-bottom-left-shape");
    assert_equals(getComputedStyle(el_with_negatives).cornerBottomRightShape, "round", "corner-bottom-right-shape");
  }, "None of the listed properties should be applied");
</script>
</body>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.style.not_allowed_here",
      "message": "Element “style” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 327,
        "byte_start": 320,
        "col": 1,
        "line": 9
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “geolocation” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 916,
        "byte_start": 894,
        "col": 1,
        "line": 31
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “geolocation” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 916,
        "byte_start": 894,
        "col": 1,
        "line": 31
      }
    },
    {
      "category": "Html",
      "code": "html.head.title.missing",
      "message": "Element “head” is missing a required instance of child element “title”.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/permission-element/geolocation-element/invalid-css-properties.tentative.html"
}
```
