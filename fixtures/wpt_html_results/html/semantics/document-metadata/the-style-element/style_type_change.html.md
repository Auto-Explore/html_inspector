# html/semantics/document-metadata/the-style-element/style_type_change.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/document-metadata/the-style-element/style_type_change.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
  <head>
    <meta charset="utf-8">
    <title>Dynamically changing HTMLStyleElement.type should change the rendering accordingly</title>
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
    <link rel="help" href="https://html.spec.whatwg.org/multipage/#the-style-element">
    <style type="no/mime">
      body { color: green }
    </style>
  </head>
  <body>
    Text content.
    <script>
      var style = document.querySelector("style");
      test(function() {
        assert_equals(document.styleSheets.length, 0);
      }, "Check initial styleSheets length type");

      test(function() {
        assert_not_equals(getComputedStyle(document.querySelector("body")).color, "rgb(0, 128, 0)");
        assert_equals(document.styleSheets.length, 0);
        style.type = "text/css";
        assert_equals(getComputedStyle(document.querySelector("body")).color, "rgb(0, 128, 0)");
        assert_equals(document.styleSheets.length, 1);
      }, "Change type from invalid type to valid type");

      test(function() {
        assert_equals(getComputedStyle(document.querySelector("body")).color, "rgb(0, 128, 0)");
        assert_equals(document.styleSheets.length, 1);
        style.type = "no/mime";
        assert_not_equals(getComputedStyle(document.querySelector("body")).color, "rgb(0, 128, 0)");
        assert_equals(document.styleSheets.length, 0);
      }, "Change type from valid type to invalid type");

    </script>
  </body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.style.type.text_css_only",
      "message": "The only allowed value for the “type” attribute for the “style” element is “text/css” (with no parameters). (But the attribute is not needed and should be omitted altogether.)",
      "severity": "Warning",
      "span": {
        "byte_end": 388,
        "byte_start": 366,
        "col": 5,
        "line": 9
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
  "source_name": "html/semantics/document-metadata/the-style-element/style_type_change.html"
}
```
