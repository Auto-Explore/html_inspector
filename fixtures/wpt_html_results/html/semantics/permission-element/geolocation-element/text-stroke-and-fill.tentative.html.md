# html/semantics/permission-element/geolocation-element/text-stroke-and-fill.tentative.html

Counts:
- errors: 1
- warnings: 5
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/permission-element/geolocation-element/text-stroke-and-fill.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<link rel="match" href="text-stroke-and-fill-ref.html">
<link rel="help" href="https://github.com/WICG/PEPC/blob/main/explainer.md#locking-the-pepc-style">
<body>
  <style>
    geolocation, div {
      -webkit-text-stroke-width: 10px;
      -webkit-text-stroke-color: blue;
      -webkit-text-fill-color: red;
    }
  </style>
  <div>
    <geolocation></geolocation>
  </div>
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
        "byte_end": 209,
        "byte_start": 202,
        "col": 3,
        "line": 6
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “geolocation” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 389,
        "byte_start": 376,
        "col": 5,
        "line": 14
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “geolocation” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 389,
        "byte_start": 376,
        "col": 5,
        "line": 14
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “html”.",
      "severity": "Error",
      "span": {
        "byte_end": 428,
        "byte_start": 421,
        "col": 1,
        "line": 17
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
  "source_name": "html/semantics/permission-element/geolocation-element/text-stroke-and-fill.tentative.html"
}
```
