# html/semantics/permission-element/geolocation-element/quirks-mode-no-height-is-still-bounded-ref.html

Counts:
- errors: 1
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/permission-element/geolocation-element/quirks-mode-no-height-is-still-bounded-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<html>
<!-- Quirks mode used deliberately -->
<meta charset=utf-8>
<style>
  geolocation {
    font-size: 14px;
    height: fit-content;
  }
</style>
<body>
  <geolocation id="geolocation"></geolocation>
  <script>
    let el = document.getElementById("geolocation");
    el.style.height = (el.clientHeight + 20) + "px";
  </script>
</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.start_tag_without_doctype",
      "message": "Start tag seen without seeing a doctype first. Expected “<!DOCTYPE html>”.",
      "severity": "Error",
      "span": {
        "byte_end": 6,
        "byte_start": 0,
        "col": 1,
        "line": 1
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “geolocation” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 189,
        "byte_start": 159,
        "col": 3,
        "line": 11
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “geolocation” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 189,
        "byte_start": 159,
        "col": 3,
        "line": 11
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
  "source_name": "html/semantics/permission-element/geolocation-element/quirks-mode-no-height-is-still-bounded-ref.html"
}
```
