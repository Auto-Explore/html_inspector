# html/semantics/permission-element/bounded-css-properties-reftest-ref.html

Counts:
- errors: 0
- warnings: 7
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/permission-element/bounded-css-properties-reftest-ref.html",
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
    The permission element should have some limits for specific properties:
    <ul>
    <li>font-weight is adjusted to be at least 200.</li>
    <li>font-style should only have "normal" or "italic" values.</li>
    <li>word-spacing should be at most 0.5 of the font size, and non-negative.</li>
    <li>letter-spacing should be between -0.05 and 0.2 of the font size.</li>
    </ul>
  </div>

<style>
  #id1 {
    font-weight: 200;
    font-style: normal;
    word-spacing: 0.5em;
    font-size: 100px;
    height: auto;
    width: auto;
    letter-spacing: 20px;
  }
  #id2 {
    word-spacing: 0em;
    height: auto;
    width: auto;
    font-size: 10px;
    letter-spacing: -0.5px;
  }
</style>

<permission id="id1" type="geolocation"></permission>
<permission id="id2" type="camera"></permission>
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
        "byte_end": 453,
        "byte_start": 446,
        "col": 1,
        "line": 14
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “permission” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 791,
        "byte_start": 751,
        "col": 1,
        "line": 33
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “permission” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 791,
        "byte_start": 751,
        "col": 1,
        "line": 33
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “permission” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 840,
        "byte_start": 805,
        "col": 1,
        "line": 34
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “permission” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 840,
        "byte_start": 805,
        "col": 1,
        "line": 34
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
  "source_name": "html/semantics/permission-element/bounded-css-properties-reftest-ref.html"
}
```
