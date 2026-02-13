# html/semantics/permission-element/geolocation-element/inherited-css-ref.html

Counts:
- errors: 1
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/permission-element/geolocation-element/inherited-css-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<link rel="help" href="https://github.com/WICG/PEPC/blob/main/explainer.md#locking-the-pepc-style">
<body>
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
      "code": "html.unknown_element.not_allowed",
      "message": "Element “geolocation” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 169,
        "byte_start": 156,
        "col": 5,
        "line": 6
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “geolocation” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 169,
        "byte_start": 156,
        "col": 5,
        "line": 6
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “html”.",
      "severity": "Error",
      "span": {
        "byte_end": 208,
        "byte_start": 201,
        "col": 1,
        "line": 9
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
  "source_name": "html/semantics/permission-element/geolocation-element/inherited-css-ref.html"
}
```
