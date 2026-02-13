# html/semantics/permission-element/geolocation-element/negative-offset.tentative.html

Counts:
- errors: 0
- warnings: 17
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/permission-element/geolocation-element/negative-offset.tentative.html",
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
<!--The geolocation element should not allow setting negative outline-offset.
-->
<style>
  #id1 {
    outline-offset: -50px;
  }
  #id2 {
    outline-offset: 50px;
  }

  /* These various expressions all result in a negative value when calculated */
  #id3 {
    outline-offset: min(-50px, 50px);
  }
  #id4 {
    outline-offset: min(10%, -50px);
  }
  #id5 {
    outline-offset: clamp(-100px, 1vw, -50px);
  }
  #id6 {
    outline-offset: 1% - 10000px;
  }
  #id7 {
    outline-offset: max(min(-1em, 10em), -5%);
  }
</style>

<geolocation id="id1"></geolocation>
<geolocation id="id2"></geolocation>
<geolocation id="id3"></geolocation>
<geolocation id="id4"></geolocation>
<geolocation id="id5"></geolocation>
<geolocation id="id6"></geolocation>
<geolocation id="id7"></geolocation>

<script>
  test(function(){
    var el_with_negatives = document.getElementById("id1");
    assert_equals(getComputedStyle(el_with_negatives).outlineOffset, "0px", "outline-offset");
  }, "Negative offset should be changed to 0px");

  test(function(){
    var el_with_positives = document.getElementById("id2");
    assert_equals(getComputedStyle(el_with_positives).outlineOffset, "50px", "outline-offset");
  }, "Positive offset are unaffected");

  test(function(){
    var el_with_negative_expr = document.getElementById("id3");
    assert_equals(getComputedStyle(el_with_negative_expr).outlineOffset, "0px", "outline-offset");
  }, "Expressions offset min(-50px, 50px) should return at least 0px");

  test(function(){
    var el_with_negative_expr = document.getElementById("id4");
    assert_equals(getComputedStyle(el_with_negative_expr).outlineOffset, "0px", "outline-offset");
  }, "Expressions offset outline-offset: min(10%, -50px) should return at least 0px");

  test(function(){
    var el_with_negative_expr = document.getElementById("id5");
    assert_equals(getComputedStyle(el_with_negative_expr).outlineOffset, "0px", "outline-offset");
  }, "Expressions offset clamp(-100px, 1vw, -50px) should return at least 0px");

  test(function(){
    var el_with_negative_expr = document.getElementById("id6");
    assert_equals(getComputedStyle(el_with_negative_expr).outlineOffset, "0px", "outline-offset");
  }, "Expressions offset 1% - 10000px should return at least 0px");

  test(function(){
    var el_with_negative_expr = document.getElementById("id7");
    assert_equals(getComputedStyle(el_with_negative_expr).outlineOffset, "0px", "outline-offset");
  }, "Expressions offset max(min(-1em, 10em), -5%) should return at least 0px");
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
        "byte_end": 339,
        "byte_start": 332,
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
        "byte_end": 801,
        "byte_start": 779,
        "col": 1,
        "line": 35
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “geolocation” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 801,
        "byte_start": 779,
        "col": 1,
        "line": 35
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “geolocation” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 838,
        "byte_start": 816,
        "col": 1,
        "line": 36
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “geolocation” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 838,
        "byte_start": 816,
        "col": 1,
        "line": 36
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “geolocation” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 875,
        "byte_start": 853,
        "col": 1,
        "line": 37
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “geolocation” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 875,
        "byte_start": 853,
        "col": 1,
        "line": 37
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “geolocation” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 912,
        "byte_start": 890,
        "col": 1,
        "line": 38
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “geolocation” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 912,
        "byte_start": 890,
        "col": 1,
        "line": 38
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “geolocation” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 949,
        "byte_start": 927,
        "col": 1,
        "line": 39
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “geolocation” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 949,
        "byte_start": 927,
        "col": 1,
        "line": 39
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “geolocation” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 986,
        "byte_start": 964,
        "col": 1,
        "line": 40
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “geolocation” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 986,
        "byte_start": 964,
        "col": 1,
        "line": 40
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “geolocation” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1023,
        "byte_start": 1001,
        "col": 1,
        "line": 41
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “geolocation” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 1023,
        "byte_start": 1001,
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
  "source_name": "html/semantics/permission-element/geolocation-element/negative-offset.tentative.html"
}
```
