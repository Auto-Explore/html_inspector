# html/semantics/permission-element/permission-icon/icon-css-property-margin-bounded-reftest.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/permission-element/permission-icon/icon-css-property-margin-bounded-reftest.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>
  The margin-inline-end is set to the exact maximum allowed limit in the ref.
  Increasing it further should have no effect.
</title>
<!-- TODO: Update the link to the permission icon spec -->
<link rel="help" href="https://github.com/WICG/PEPC/blob/main/explainer.md" />
<link rel="match" href="icon-css-property-margin-bounded-reftest-ref.html" />
<style>
  permission {
    font-size: 10px;
  }
  ::permission-icon {
    margin-inline-end: 100px;
  }
</style>
<permission id="geolocation" type="geolocation"></permission>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “permission” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 535,
        "byte_start": 487,
        "col": 1,
        "line": 17
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “permission” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 535,
        "byte_start": 487,
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
  "source_name": "html/semantics/permission-element/permission-icon/icon-css-property-margin-bounded-reftest.html"
}
```
