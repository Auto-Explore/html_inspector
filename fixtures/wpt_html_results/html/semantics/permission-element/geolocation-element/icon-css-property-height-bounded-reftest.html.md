# html/semantics/permission-element/geolocation-element/icon-css-property-height-bounded-reftest.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/permission-element/geolocation-element/icon-css-property-height-bounded-reftest.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>
  The icon is set to the exact maximum height in the reference.
  Increasing it further should have no effect.
</title>
<!-- TODO: Update the link to the permission icon spec -->
<link rel="help" href="https://github.com/WICG/PEPC/blob/main/explainer.md" />
<link rel="match" href="icon-css-property-height-bounded-reftest-ref.html" />
<style>
  geolocation {
    font-size: 10px;
  }
  ::permission-icon {
    height: 100px;
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
        "byte_end": 493,
        "byte_start": 463,
        "col": 1,
        "line": 17
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “geolocation” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 493,
        "byte_start": 463,
        "col": 1,
        "line": 17
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
  "source_name": "html/semantics/permission-element/geolocation-element/icon-css-property-height-bounded-reftest.html"
}
```
