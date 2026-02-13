# html/semantics/document-metadata/the-style-element/style_non_matching_media.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/document-metadata/the-style-element/style_non_matching_media.html",
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
    <title>HTML Test: Non-matching media type should have stylesheet</title>
    <link rel="help" href="https://html.spec.whatwg.org/multipage/#the-style-element">
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
    <style media="unknown">
      body { color: green }
    </style>
  </head>
  <body>
    <script>
      test(function() {
        assert_equals(document.styleSheets.length, 1);
      });
    </script>
  </body>
</html>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/document-metadata/the-style-element/style_non_matching_media.html"
}
```
