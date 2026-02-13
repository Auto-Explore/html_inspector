# html/dom/elements/global-attributes/dataset-delete.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/elements/global-attributes/dataset-delete.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
  <head>
    <title>Dataset - Delete</title>
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
  </head>
  <body>
    <h1>Dataset - Delete</h1>
    <div id="log"></div>
    <script>
      function testDelete(attr, prop)
      {
        var d = document.createElement("div");
        d.setAttribute(attr, "value");
        delete d.dataset[prop];
        return d.hasAttribute(attr) === false && d.getAttribute(attr) != "value";
      }

      function testDeleteNoAdd(prop)
      {
        var d = document.createElement("div");
        delete d.dataset[prop];
        return true;
      }

      test(function() { assert_true(testDelete('data-foo', 'foo')); },
        "Deleting element.dataset['foo'] should also remove an attribute with name 'data-foo' should it exist.");
      test(function() { assert_true(testDelete('data-foo-bar', 'fooBar')); },
        "Deleting element.dataset['fooBar'] should also remove an attribute with name 'data-foo-bar' should it exist.");
      test(function() { assert_true(testDelete('data--', '-')); },
        "Deleting element.dataset['-'] should also remove an attribute with name 'data--' should it exist.");
      test(function() { assert_true(testDelete('data--foo', 'Foo')); },
        "Deleting element.dataset['Foo'] should also remove an attribute with name 'data--foo' should it exist.");
      test(function() {
        var d = document.createElement("div");
        d.setAttribute('data--foo', "value");
        assert_equals(d.dataset['-foo'], undefined);
        assert_false('-foo' in d.dataset);
        delete d.dataset['-foo'];
        assert_true(d.hasAttribute('data--foo'));
        assert_equals(d.getAttribute('data--foo'), "value");
      }, "Deleting element.dataset['-foo'] should not remove an attribute with name 'data--foo' should it exist.");
      test(function() { assert_true(testDelete('data---foo', '-Foo')); },
        "Deleting element.dataset['-Foo'] should also remove an attribute with name 'data---foo' should it exist.");
      test(function() { assert_true(testDelete('data-', '')); },
        "Deleting element.dataset[''] should also remove an attribute with name 'data-' should it exist.");
      test(function() { assert_true(testDelete('data-\xE0', '\xE0')); },
        "Deleting element.dataset['\xE0'] should also remove an attribute with name 'data-\xE0' should it exist.");
      test(function() { assert_true(testDeleteNoAdd('foo')); },
        "Deleting element.dataset['foo'] should not throw if even if the element does now have an attribute with the name data-foo.");
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
  "source_name": "html/dom/elements/global-attributes/dataset-delete.html"
}
```
