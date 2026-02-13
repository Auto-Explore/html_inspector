# html/semantics/permission-element/geolocation-element/icon-css-property-stroke-reftest.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/permission-element/geolocation-element/icon-css-property-stroke-reftest.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>The icon of the element should change if the stroke is changed</title>
<!-- TODO: Update the link to the permission icon spec -->
<link rel="help" href="https://github.com/WICG/PEPC/blob/main/explainer.md">
<link rel="mismatch" href="standard-location-element-ref.html">
<style>
  ::permission-icon {
    stroke: red;
  }
</style>
<geolocation id="geolocation"></geolocation>
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
        "byte_end": 384,
        "byte_start": 354,
        "col": 1,
        "line": 11
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “geolocation” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 384,
        "byte_start": 354,
        "col": 1,
        "line": 11
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
  "source_name": "html/semantics/permission-element/geolocation-element/icon-css-property-stroke-reftest.html"
}
```
