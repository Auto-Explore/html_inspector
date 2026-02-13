# html/semantics/permission-element/border-radius-identical.tentative.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/permission-element/border-radius-identical.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Permission Element Border Radius Clamping</title>
<link rel="help"
    href="https://github.com/WICG/PEPC/blob/main/explainer.md#locking-the-pepc-style"/>
<link rel="match" href="border-radius-identical-ref.html">
<p>This test verifies that a large border-radius on a permission element is clamped to half of the element's smallest dimension, resulting in a pill shape.</p>
<style>
  permission {
    width: 100px;
    height: 40px;
    border-radius: 1000px;
    background-color: blue;
    display: inline-block;
  }
</style>
<permission type="geolocation"></permission>
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
        "byte_end": 427,
        "byte_start": 420,
        "col": 1,
        "line": 8
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “permission” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 605,
        "byte_start": 574,
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
        "byte_end": 605,
        "byte_start": 574,
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
  "source_name": "html/semantics/permission-element/border-radius-identical.tentative.html"
}
```
