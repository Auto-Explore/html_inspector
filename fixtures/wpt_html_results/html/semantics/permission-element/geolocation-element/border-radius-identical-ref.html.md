# html/semantics/permission-element/geolocation-element/border-radius-identical-ref.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/permission-element/geolocation-element/border-radius-identical-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Geolocation Element Border Radius Clamping Reference</title>
<p>This test verifies that a large border-radius on a geolocation element is clamped to half of the element's smallest dimension, resulting in a pill shape.</p>
<style>
  geolocation {
    width: 100px;
    height: 40px;
    border-radius: 25px;
    background-color: blue;
    display: inline-block;
  }
</style>
<geolocation></geolocation>
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
        "byte_end": 275,
        "byte_start": 268,
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
        "byte_end": 434,
        "byte_start": 421,
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
        "byte_end": 434,
        "byte_start": 421,
        "col": 1,
        "line": 14
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/permission-element/geolocation-element/border-radius-identical-ref.html"
}
```
