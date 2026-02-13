# html/semantics/permission-element/large-min-size-reftest.tentative.html

Counts:
- errors: 0
- warnings: 7
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/permission-element/large-min-size-reftest.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<link rel="match" href="large-min-size-reftest-ref.html">
<link rel="help" href="https://github.com/WICG/PEPC/blob/main/explainer.md#locking-the-pepc-style">
<body>
  <div>
    The permission element min-width/height should not be allowed to exceeed the maximum
    allowed value for max-width/height.
  </div>

<style>
  #id1 {
    font-size: 10px;
    min-height: 100px;
    min-width: 1000px;
    border: 0px;

    width: 1px;
    height: 1px;
  }
  #id2 {
    font-size: 10px;
    max-height: 5px;
    max-width: 10px;
    border: 0px;

    width: 1000px;
    height: 1000px;
  }
</style>

<div><permission id="id1" type="geolocation"></permission></div>
<div><permission id="id2" type="camera"></permission></div>
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
        "byte_end": 356,
        "byte_start": 349,
        "col": 1,
        "line": 11
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “permission” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 676,
        "byte_start": 636,
        "col": 6,
        "line": 32
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “permission” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 676,
        "byte_start": 636,
        "col": 6,
        "line": 32
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “permission” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 736,
        "byte_start": 701,
        "col": 6,
        "line": 33
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “permission” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 736,
        "byte_start": 701,
        "col": 6,
        "line": 33
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
  "source_name": "html/semantics/permission-element/large-min-size-reftest.tentative.html"
}
```
