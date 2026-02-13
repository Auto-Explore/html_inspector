# html/semantics/permission-element/unbounded-width-with-border-reftest-ref.html

Counts:
- errors: 0
- warnings: 13
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/permission-element/unbounded-width-with-border-reftest-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<meta charset=utf-8>
<body>
  <div>
    The permission element width is allowed to be unbounded if there is a border which makes the
    bounds of the permission element sufficiently clear.
    The border should have enough width, enough contrast with the background-color and no transparency.
  </div>

  <style>
    #unlimited-width {
      font-size: 10px;

      background-color: white;
      border: solid 1px black;
      width: 500px;
    }
    #no-border-width {
      font-size: 10px;
      background-color: white;
      border: 0px;

      /* Used to compute width which will then be set in JS */
      width: fit-content;
    }
    #no-contrast {
      font-size: 10px;
      background-color: white;
      border: solid 1px yellow;

      /* Used to compute width which will then be set in JS */
      width: fit-content;
    }
    #transparent {
      font-size: 10px;
      background-color: white;
      border: solid 1px #000000ee;

      /* Used to compute width which will then be set in JS */
      width: fit-content;
    }
    #top-no-contrast {
      font-size: 10px;
      background-color: white;
      border: solid 1px black;
      border-top-color: white;

/* Used to compute width which will then be set in JS */
width: fit-content;
    }
  </style>

<div><permission id="unlimited-width" type="geolocation"></permission></div>
<div><permission id="no-border-width" type="camera"></permission></div>
<div><permission id="no-contrast" type="microphone"></permission></div>
<div><permission id="transparent" type="microphone camera"></permission></div>
<div><permission id="top-no-contrast" type="geolocation"></permission></div>

<script>
  function setWidthToMax(el) {
    let w = getComputedStyle(el).width;
    el.style.width = `calc(${w} * 3)`;
  }

  setWidthToMax(document.getElementById("no-border-width"));
  setWidthToMax(document.getElementById("no-contrast"));
  setWidthToMax(document.getElementById("transparent"));
  setWidthToMax(document.getElementById("top-no-contrast"));
</script>
</body>
</html>
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
        "byte_end": 336,
        "byte_start": 329,
        "col": 3,
        "line": 11
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “permission” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1361,
        "byte_start": 1309,
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
        "byte_end": 1361,
        "byte_start": 1309,
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
        "byte_end": 1433,
        "byte_start": 1386,
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
        "byte_end": 1433,
        "byte_start": 1386,
        "col": 6,
        "line": 55
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “permission” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1505,
        "byte_start": 1458,
        "col": 6,
        "line": 56
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “permission” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 1505,
        "byte_start": 1458,
        "col": 6,
        "line": 56
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “permission” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1584,
        "byte_start": 1530,
        "col": 6,
        "line": 57
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “permission” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 1584,
        "byte_start": 1530,
        "col": 6,
        "line": 57
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “permission” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1661,
        "byte_start": 1609,
        "col": 6,
        "line": 58
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “permission” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 1661,
        "byte_start": 1609,
        "col": 6,
        "line": 58
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
  "source_name": "html/semantics/permission-element/unbounded-width-with-border-reftest-ref.html"
}
```
