# html/semantics/permission-element/geolocation-element/geolocation-element-with-comment.tentative.html

Counts:
- errors: 0
- warnings: 10
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/permission-element/geolocation-element/geolocation-element-with-comment.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<link rel="help" href="https://github.com/WICG/PEPC/blob/main/explainer.md" />
<link rel="match" href="geolocation-element-with-comment-ref.html" />

<geolocation> <!-- Comment 1 --> </geolocation>
<geolocation>
  <!-- C1 -->
  Text
  <!-- C2 -->
</geolocation>
<div style="display: flex">
  <geolocation> <!-- Comment 1 --> </geolocation>
</div>
<geolocation>
  <!-- Comment Before -->
  <span>
    <button>Click Me</button>
    <textarea>Some text</textarea>
  </span>
  <!-- Comment After -->
</geolocation>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “geolocation” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 179,
        "byte_start": 166,
        "col": 1,
        "line": 5
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “geolocation” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 179,
        "byte_start": 166,
        "col": 1,
        "line": 5
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “geolocation” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 227,
        "byte_start": 214,
        "col": 1,
        "line": 6
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “geolocation” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 227,
        "byte_start": 214,
        "col": 1,
        "line": 6
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “geolocation” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 321,
        "byte_start": 308,
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
        "byte_end": 321,
        "byte_start": 308,
        "col": 3,
        "line": 12
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “geolocation” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 376,
        "byte_start": 363,
        "col": 1,
        "line": 14
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “geolocation” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 376,
        "byte_start": 363,
        "col": 1,
        "line": 14
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
  "source_name": "html/semantics/permission-element/geolocation-element/geolocation-element-with-comment.tentative.html"
}
```
