# html/dom/elements/global-attributes/dataset-enumeration.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/elements/global-attributes/dataset-enumeration.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
  <head>
    <title>Dataset - Enumeration</title>
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
  </head>
  <body>
    <h1>Dataset - Enumeration</h1>
    <div id="log"></div>
    <script>
      function testEnumeration(array)
      {
        var d = document.createElement("div");
        for (var i = 0; i < array.length; ++i)
          d.setAttribute(array[i], "value");

        var count = 0;
        for (var item in d.dataset)
          count++;

        return count;
      }

      test(function() { assert_equals(testEnumeration(['data-foo', 'data-bar', 'data-baz']), 3); },
        "A dataset should be enumeratable.");
      test(function() { assert_equals(testEnumeration(['data-foo', 'data-bar', 'dataFoo']), 2); },
        "Only attributes who qualify as dataset properties should be enumeratable in the dataset.");
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
  "source_name": "html/dom/elements/global-attributes/dataset-enumeration.html"
}
```
