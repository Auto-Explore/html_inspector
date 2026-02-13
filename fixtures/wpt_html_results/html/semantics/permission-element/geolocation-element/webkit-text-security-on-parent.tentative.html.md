# html/semantics/permission-element/geolocation-element/webkit-text-security-on-parent.tentative.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/permission-element/geolocation-element/webkit-text-security-on-parent.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Geolocation Element with -webkit-text-security on parent</title>
<link rel="help" href="https://github.com/WICG/PEPC">
<link rel="match" href="webkit-text-security-on-parent-ref.html">
<style>
  #container {
    -webkit-text-security: disc;
  }
</style>

<div id="container"><geolocation id="p"></geolocation></div>
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
        "byte_end": 341,
        "byte_start": 321,
        "col": 21,
        "line": 12
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “geolocation” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 341,
        "byte_start": 321,
        "col": 21,
        "line": 12
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
  "source_name": "html/semantics/permission-element/geolocation-element/webkit-text-security-on-parent.tentative.html"
}
```
