# html/rendering/non-replaced-elements/form-controls/placeholder-opacity-default.tentative.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/form-controls/placeholder-opacity-default.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
  <head>
    <meta charset="utf-8" />
    <title>Placeholder Test: opacity default value</title>
    <link rel="author" title="Karl Dubost" href="mailto:kdubost@mozilla.com"
    />
    <link rel="help" href="https://drafts.csswg.org/css-pseudo-4/#placeholder-pseudo"
    />
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
  </head>
  <body>
    <input
      id="opacity"
      placeholder="::placeholder should have default opacity: 1"
    />
    <script>
      test(function () {
        var target = document.getElementById("opacity");
        assert_equals(getComputedStyle(target, '::placeholder').opacity, "1");
      }, "Default opacity value is '1'");
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
  "source_name": "html/rendering/non-replaced-elements/form-controls/placeholder-opacity-default.tentative.html"
}
```
