# html/dom/elements/global-attributes/dataset-get.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/elements/global-attributes/dataset-get.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
  <head>
    <title>Dataset - Get</title>
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
  </head>
  <body>
    <h1>Dataset - Get</h1>
    <div id="log"></div>
    <script>
      function testGet(attr, expected)
      {
        var d = document.createElement("div");
        d.setAttribute(attr, "value");
        return d.dataset[expected] == "value";
      }

      test(function() { assert_true(testGet('data-foo', 'foo')); },
        "Getting element.dataset['foo'] should return the value of element.getAttribute('data-foo')'");
      test(function() { assert_true(testGet('data-foo-bar', 'fooBar')); },
        "Getting element.dataset['fooBar'] should return the value of element.getAttribute('data-foo-bar')'");
      test(function() { assert_true(testGet('data--', '-')); },
        "Getting element.dataset['-'] should return the value of element.getAttribute('data--')'");
      test(function() { assert_true(testGet('data--foo', 'Foo')); },
        "Getting element.dataset['Foo'] should return the value of element.getAttribute('data--foo')'");
      test(function() { assert_true(testGet('data---foo', '-Foo')); },
        "Getting element.dataset['-Foo'] should return the value of element.getAttribute('data---foo')'");
      test(function() { assert_true(testGet('data-Foo', 'foo')); },
        "Getting element.dataset['foo'] should return the value of element.getAttribute('data-Foo')'");
      test(function() { assert_true(testGet('data-', '')); },
        "Getting element.dataset[''] should return the value of element.getAttribute('data-')'");
      test(function() { assert_true(testGet('data-\xE0', '\xE0')); },
        "Getting element.dataset['\xE0'] should return the value of element.getAttribute('data-\xE0')'");
      test(function() { assert_true(testGet('data-to-string', 'toString')); },
        "Getting element.dataset['toString'] should return the value of element.getAttribute('data-to-string')'");

      function matchesNothingInDataset(attr)
      {
        var d = document.createElement("div");
        d.setAttribute(attr, "value");

        if (!d.dataset)
          return false;

        var count = 0;
        for (var item in d.dataset)
          count++;
        return count == 0;
      }

      test(function() { assert_true(matchesNothingInDataset('dataFoo')); },
        "Tests that an attribute named dataFoo does not make an entry in the dataset DOMStringMap.");

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
  "source_name": "html/dom/elements/global-attributes/dataset-get.html"
}
```
