# html/semantics/permission-element/geolocation-element/no-children-rendered.tentative.html

Counts:
- errors: 0
- warnings: 10
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/permission-element/geolocation-element/no-children-rendered.tentative.html",
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
<!-- The geolocation element is not a void element. Its children do not render. -->

<!-- This tests for implementations that used to have no end tag for the geolocation element
 In those implementations there would be 2 geolocation elements in this div. -->
<div id="geolocation-element-div">
  <geolocation>
  <geolocation>
</div>

<geolocation id="el1">This is some text</geolocation>

<!-- The geolocation element does not automatically close <p> tags -->
<p id="paragraph">Foo <geolocation>bar</geolocation> baz</p>

<script>
  test(function(){
    assert_equals(1, document.getElementById("geolocation-element-div").childElementCount);
    assert_equals('', document.getElementById("el1").innerText);
    assert_equals('This is some text', document.getElementById("el1").textContent);
    assert_equals('This is some text', document.getElementById("el1").innerHTML);
    assert_equals(4, document.body.childElementCount); //div, geolocation, script, p
    assert_equals('Foo bar baz', document.getElementById("paragraph").textContent);
  }, "The geolocation element should have no end tag or contents");
</script>
</body>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “geolocation” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 559,
        "byte_start": 546,
        "col": 3,
        "line": 12
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “geolocation” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 559,
        "byte_start": 546,
        "col": 3,
        "line": 12
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “geolocation” not allowed as child of “geolocation” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 575,
        "byte_start": 562,
        "col": 3,
        "line": 13
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “geolocation” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 575,
        "byte_start": 562,
        "col": 3,
        "line": 13
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “geolocation” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 606,
        "byte_start": 584,
        "col": 1,
        "line": 16
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “geolocation” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 606,
        "byte_start": 584,
        "col": 1,
        "line": 16
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “geolocation” not allowed as child of “p” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 745,
        "byte_start": 732,
        "col": 23,
        "line": 19
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “geolocation” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 745,
        "byte_start": 732,
        "col": 23,
        "line": 19
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
  "source_name": "html/semantics/permission-element/geolocation-element/no-children-rendered.tentative.html"
}
```
