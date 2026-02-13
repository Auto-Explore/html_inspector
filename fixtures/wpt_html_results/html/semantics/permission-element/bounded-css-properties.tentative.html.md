# html/semantics/permission-element/bounded-css-properties.tentative.html

Counts:
- errors: 0
- warnings: 9
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/permission-element/bounded-css-properties.tentative.html",
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
<!--The permission element should have some limits for specific properties:
  * font-weight is adjusted to be at least 200.
  * font-style should only have "normal" or "italic" values.
  * word-spacing should be at most 0.5 of the font size, and non-negative.
  * letter-spacing should be between -0.05 and 0.2 of the font size.
-->
<style>
  #id-over-bounds {
    font-weight: 100;
    font-style: oblique 30deg;
    word-spacing: 1em;
    font-size: 100px;
    letter-spacing: 21px;
    box-shadow: 5px 5px inset;
  }
  #id-under-bounds {
    word-spacing: -1px;
    font-size: 100px;
    letter-spacing: -6px;
    box-shadow: rgb(255, 0, 0) 5px 4px 3px 2px, 5px 5px inset;
  }
  #id-within-bounds {
    font-weight: 300;
    font-style: italic;
    word-spacing: 0.4em;
    font-size: 100px;
    letter-spacing: 15px;
    box-shadow: rgb(255, 0, 0) 5px 4px 3px 2px;
  }
</style>


<permission id="id-over-bounds" type="geolocation"></permission>
<permission id="id-under-bounds" type="camera"></permission>
<permission id="id-within-bounds" type="microphone"></permission>

<script>
  test(function(){
    var el = document.getElementById("id-over-bounds");
    assert_equals(getComputedStyle(el).fontWeight, "200", "font-weight");
    assert_equals(getComputedStyle(el).fontStyle, "normal", "font-style");
    assert_equals(getComputedStyle(el).wordSpacing, "50px", "word-spacing");
    assert_equals(getComputedStyle(el).letterSpacing, "20px", "letter-spacing");
    assert_equals(getComputedStyle(el).boxShadow, "none", "box-shadow");

    el = document.getElementById("id-under-bounds");
    assert_equals(getComputedStyle(el).wordSpacing, "0px", "word-spacing, negative");
    assert_equals(getComputedStyle(el).letterSpacing, "-5px", "letter-spacing, negative");
    assert_equals(getComputedStyle(el).boxShadow, "none", "box-shadow, multiple");
  }, "Properties with out-of-bounds values should be corrected");

  test(function(){
    var el = document.getElementById("id-within-bounds");
    assert_equals(getComputedStyle(el).fontWeight, "300", "font-weight");
    assert_equals(getComputedStyle(el).fontStyle, "italic", "font-style");
    assert_equals(getComputedStyle(el).wordSpacing, "40px", "word-spacing");
    assert_equals(getComputedStyle(el).letterSpacing, "15px", "letter-spacing");
    assert_equals(getComputedStyle(el).boxShadow, "rgb(255, 0, 0) 5px 4px 3px 2px", "box-shadow");

    el.style.letterSpacing = "-4px";
    assert_equals(getComputedStyle(el).letterSpacing, "-4px", "letter-spacing, negative");
  }, "Properties with values in bounds should not be modified");
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
        "byte_end": 590,
        "byte_start": 583,
        "col": 1,
        "line": 13
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “permission” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1185,
        "byte_start": 1134,
        "col": 1,
        "line": 39
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “permission” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 1185,
        "byte_start": 1134,
        "col": 1,
        "line": 39
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “permission” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1246,
        "byte_start": 1199,
        "col": 1,
        "line": 40
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “permission” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 1246,
        "byte_start": 1199,
        "col": 1,
        "line": 40
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “permission” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1312,
        "byte_start": 1260,
        "col": 1,
        "line": 41
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “permission” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 1312,
        "byte_start": 1260,
        "col": 1,
        "line": 41
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
  "source_name": "html/semantics/permission-element/bounded-css-properties.tentative.html"
}
```
