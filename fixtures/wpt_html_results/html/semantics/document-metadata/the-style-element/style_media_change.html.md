# html/semantics/document-metadata/the-style-element/style_media_change.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/document-metadata/the-style-element/style_media_change.html",
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
    <title>Dynamically changing HTMLStyleElement.media should change the rendering accordingly</title>
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
    <link rel="help" href="https://html.spec.whatwg.org/multipage/#the-style-element">
    <style>
      span {
       color: red;
      }
    </style>
    <style id="text-style" media="none">
      span {
       color: green;
      }
    </style>
    <style id="body-style" media="aural">
      body {
       color: green;
      }
    </style>
  </head>
  <body>
    <span>text</span>
    <script>
      test(function() {
        var element = document.querySelector("span");
        assert_equals(getComputedStyle(element).color, "rgb(255, 0, 0)");
        document.getElementById("text-style").media = 'all';
        assert_equals(getComputedStyle(element).color, "rgb(0, 128, 0)");
      }, "change media value dynamically");

      test(function() {
        var style = document.getElementById("body-style");
        assert_not_equals(getComputedStyle(document.querySelector("body")).color, "rgb(0, 128, 0)");
        style.removeAttribute("media");
        assert_equals(getComputedStyle(document.querySelector("body")).color, "rgb(0, 128, 0)");
      }, "removing media attribute");
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
  "source_name": "html/semantics/document-metadata/the-style-element/style_media_change.html"
}
```
