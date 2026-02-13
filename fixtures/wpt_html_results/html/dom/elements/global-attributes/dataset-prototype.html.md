# html/dom/elements/global-attributes/dataset-prototype.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/elements/global-attributes/dataset-prototype.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
  <head>
    <title>Dataset - element.dataset is an instance of DOMStringMap</title>
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
  </head>
  <body>
    <h1>Dataset - element.dataset is an instance of DOMStringMap</h1>
    <div id="log"></div>
    <script>
      test(function() { assert_true(document.createElement("div").dataset instanceof window.DOMStringMap); },
        "An elements dataset property is an instance of a DOMStringMap");
      test(function() {
        var dataset = document.createElement("div").dataset;
        assert_true("toString" in dataset, '"toString" in dataset');
        assert_equals(dataset.toString, Object.prototype.toString);
        assert_false("expando" in dataset, '"expando" in dataset');
        assert_equals(dataset.expando, undefined);
        Object.prototype.expando = 42;
        assert_true("expando" in dataset, '"expando" in dataset');
        assert_equals(dataset.expando, 42);
      }, "Properties on Object.prototype should shine through.");
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
  "source_name": "html/dom/elements/global-attributes/dataset-prototype.html"
}
```
