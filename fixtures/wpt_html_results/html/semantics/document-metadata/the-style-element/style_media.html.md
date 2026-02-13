# html/semantics/document-metadata/the-style-element/style_media.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/document-metadata/the-style-element/style_media.html",
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
    <title>HTML Test: The style information must be applied to the environment specified by the media attribute</title>
    <link rel="author" title="Intel" href="http://www.intel.com/">
    <link rel="help" href="https://html.spec.whatwg.org/multipage/#attr-style-media">
    <link rel="help" href="https://html.spec.whatwg.org/multipage/#the-style-element">
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
    <style>
      #test {
        width: 100px;
      }
    </style>
    <style id="style">
      #test {
        width: 50px;
      }
    </style>
  </head>
  <body>
    <div id="log"></div>
    <div id="test"></div>
    <script>
      test(function() {
        var testElement = document.getElementById("test");
        var style = document.getElementById("style");
        var width1, width2;

        width1 = window.getComputedStyle(testElement)["width"];
        assert_equals(width1, "50px", "The style should be applied.");

        style.media = "print";
        width2 = window.getComputedStyle(testElement)["width"];
        assert_equals(width2, "100px", "The style should not be applied.");
      }, "The style information must be applied to the environment specified by the media attribute");
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
  "source_name": "html/semantics/document-metadata/the-style-element/style_media.html"
}
```
