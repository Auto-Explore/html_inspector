# html/semantics/permission-element/large-min-size-reftest-ref.html

Counts:
- errors: 0
- warnings: 7
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/permission-element/large-min-size-reftest-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<body>
  <div>
    The permission element min-width/height should not be allowed to exceeed the maximum
    allowed value for max-width/height.
  </div>

<style>
  #id1 {
    font-size: 10px;
    height: 30px;
    border: 0px;
    /* Used to determine the "fit-content" width value which is used
       to set the width to 3 * fit-content.
    */
    width: fit-content;
  }
  #id2 {
    font-size: 10px;
    height: 10px;
    border: 0px;
    width: fit-content;
  }
</style>

<div><permission id="id1" type="geolocation"></permission></div>
<div><permission id="id2" type="camera"></permission></div>
<script>
  el = document.getElementById("id1");
  let w = getComputedStyle(el).width;
  el.style.width = `calc(3 * ${w})`; // 3 * fit-content
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
        "byte_end": 198,
        "byte_start": 191,
        "col": 1,
        "line": 9
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “permission” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 560,
        "byte_start": 520,
        "col": 6,
        "line": 27
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “permission” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 560,
        "byte_start": 520,
        "col": 6,
        "line": 27
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “permission” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 620,
        "byte_start": 585,
        "col": 6,
        "line": 28
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “permission” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 620,
        "byte_start": 585,
        "col": 6,
        "line": 28
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
  "source_name": "html/semantics/permission-element/large-min-size-reftest-ref.html"
}
```
