# html/semantics/permission-element/bounded-sizes-reftest.tentative.html

Counts:
- errors: 0
- warnings: 9
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/permission-element/bounded-sizes-reftest.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<link rel="match" href="bounded-sizes-reftest-ref.html">
<link rel="help" href="https://github.com/WICG/PEPC/blob/main/explainer.md#locking-the-pepc-style">
<body>
  <div>
    The permission element should have some limits for the min/max-width/height:
    <ul>
    <li>min-width should be sufficient to fit the element text (depends on user agent implementation)</li>
    <li>max-width should be at most 3x min-width</li>
    <li>min-height should be sufficient to fit the element text (1em)</li>
    <li>max-height should be at most 3x min-height</li>
    <li>padding-left/top only work with width/height: auto and are at most 5em/1em</li>
    <li>padding-right/bottom are copied over from padding-left/top in this case</li>
    </ul>
  </div>

<style>
  #id1 {
    font-size: 10px;
    min-height: 1px;
    max-height: 100px;
    border: 0px;

    /* These values are extreme enough that they should be out of bounds for any implementation */
    min-width: 10px;
    max-width: 1000px;

    /* This element will be as tiny as possible */
    width: 1px;
    height: 1px;
  }
  #id2 {
    font-size: 10px;
    min-height: 1px;
    max-height: 100px;
    border: 0px;

    /* These values are extreme enough that they should be out of bounds for any implementation */
    min-width: 10px;
    max-width: 1000px;

    /* This element will be as large as possible */
    width: 1000px;
    height: 1000px;
  }
  #id3 {
    font-size: 10px;
    width: auto;
    height: auto;
    border: 0px;

    /* There is a slight misalignment of the text (by 1px) when using
       padding vs using width/height. Since this test's purpose is to
       check that the bounds are identical, the color and background-color
       are set to the same value to cover the slight text misalignment */
    color:black;
    background-color: black;
    border: 1em solid darkmagenta;

    /* Only padding-top and padding-left are taken into account */
    padding-top: 5px;
    padding-left: 1000px;
    padding-bottom: 1px;
    padding-right: 1px;
  }
</style>

<div><permission id="id1" type="geolocation"></permission></div>
<div><permission id="id2" type="camera"></permission></div>
<div><permission id="id3" type="microphone"></permission></div>
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
        "byte_end": 791,
        "byte_start": 784,
        "col": 1,
        "line": 18
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “permission” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 2124,
        "byte_start": 2084,
        "col": 6,
        "line": 69
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “permission” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 2124,
        "byte_start": 2084,
        "col": 6,
        "line": 69
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “permission” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 2184,
        "byte_start": 2149,
        "col": 6,
        "line": 70
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “permission” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 2184,
        "byte_start": 2149,
        "col": 6,
        "line": 70
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “permission” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 2248,
        "byte_start": 2209,
        "col": 6,
        "line": 71
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “permission” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 2248,
        "byte_start": 2209,
        "col": 6,
        "line": 71
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
  "source_name": "html/semantics/permission-element/bounded-sizes-reftest.tentative.html"
}
```
