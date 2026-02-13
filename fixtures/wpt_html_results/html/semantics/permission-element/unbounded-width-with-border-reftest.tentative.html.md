# html/semantics/permission-element/unbounded-width-with-border-reftest.tentative.html

Counts:
- errors: 0
- warnings: 13
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/permission-element/unbounded-width-with-border-reftest.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<link rel="match" href="unbounded-width-with-border-reftest-ref.html">
<link rel="help" href="https://github.com/WICG/PEPC/blob/main/explainer.md#locking-the-pepc-style">
<body>
  <div>
    The permission element width is allowed to be unbounded if there is a border which makes the
    bounds of the permission element sufficiently clear.
    The border should have enough width, enough contrast with the background-color and no transparency.
  </div>

<style>
  /* This element passes all checks and therefore its width is not limited */
  #unlimited-width {
    font-size: 10px;
    background-color: white;
    border: solid 1px black;
    width: 500px;
  }
  /* This element's border width is too small */
  #no-border-width {
    font-size: 10px;
    background-color: white;
    border: 0px;
    width: 500px;
  }
  /* This element's border color is not sufficiently different from the background-color */
  #no-contrast {
    font-size: 10px;
    background-color: white;
    border: solid 1px yellow;
    width: 500px;
  }
  /* This element's border color has some transparency (alpha < 1) */
  #transparent {
    font-size: 10px;
    background-color: white;
    border: solid 1px #000000ee;
    width: 500px;
  }
  /* This element's border is good except the top part which has insuficient contrast */
  #top-no-contrast {
    font-size: 10px;
    background-color: white;
    border: solid 1px black;
    border-top-color: white;
    width: 500px;
  }
</style>

<div><permission id="unlimited-width" type="geolocation"></permission></div>
<div><permission id="no-border-width" type="camera"></permission></div>
<div><permission id="no-contrast" type="microphone"></permission></div>
<div><permission id="transparent" type="microphone camera"></permission></div>
<div><permission id="top-no-contrast" type="geolocation"></permission></div>

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
        "byte_end": 498,
        "byte_start": 491,
        "col": 1,
        "line": 12
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “permission” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1568,
        "byte_start": 1516,
        "col": 6,
        "line": 51
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “permission” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 1568,
        "byte_start": 1516,
        "col": 6,
        "line": 51
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “permission” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1640,
        "byte_start": 1593,
        "col": 6,
        "line": 52
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “permission” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 1640,
        "byte_start": 1593,
        "col": 6,
        "line": 52
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “permission” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1712,
        "byte_start": 1665,
        "col": 6,
        "line": 53
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “permission” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 1712,
        "byte_start": 1665,
        "col": 6,
        "line": 53
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “permission” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1791,
        "byte_start": 1737,
        "col": 6,
        "line": 54
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “permission” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 1791,
        "byte_start": 1737,
        "col": 6,
        "line": 54
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “permission” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1868,
        "byte_start": 1816,
        "col": 6,
        "line": 55
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “permission” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 1868,
        "byte_start": 1816,
        "col": 6,
        "line": 55
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
  "source_name": "html/semantics/permission-element/unbounded-width-with-border-reftest.tentative.html"
}
```
