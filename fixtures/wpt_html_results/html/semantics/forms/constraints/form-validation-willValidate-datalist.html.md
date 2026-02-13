# html/semantics/forms/constraints/form-validation-willValidate-datalist.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/constraints/form-validation-willValidate-datalist.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>The constraint validation API Test: element.willValidate</title>
<link rel="author" title="Intel" href="http://www.intel.com/">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#dom-cva-willvalidate">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#the-constraint-validation-api">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<datalist></datalist>
<script>
  var dl = document.querySelector("datalist");
  function runTest(element, name) {
    test(function () {
      assert_true(element.willValidate, "The willValidate attribute should be true initially.");

      dl.appendChild(element);
      assert_false(element.willValidate, "The willValidate attribute should be false if element has datalist parent.");

      element.remove();
      assert_true(element.willValidate, "The willValidate attribute should be true if element has no parent.");

      let div = document.createElement("div");
      div.appendChild(element);
      let foo = document.createElementNS('some-random-namespace', 'foo');
      foo.appendChild(div);
      dl.appendChild(foo);
      assert_false(element.willValidate, "The willValidate attribute should be false if element has datalist ancestor.");

      foo.remove();
      assert_true(element.willValidate, "The willValidate attribute should be true if element has no datalist ancestor.");
    }, name);
  }

  var testElements = [
    {
      tag: "input",
      types: ["text", "search", "tel", "url", "email", "password", "datetime-local", "date", "month", "week", "time", "color", "file", "submit"]
    },
    {
      tag: "button",
      types: ["submit"],
    },
    {
      tag: "select",
      types: [],
    },
    {
      tag: "textarea",
      types: [],
    }
  ].forEach(function(testData) {
    if (testData.types.length > 0) {
      testData.types.forEach(function(type) {
        let ele = document.createElement(testData.tag);
        try {
          ele.type = type;
        } catch (e) {
          //Do nothing, avoid the runtime error breaking the test
        }
        runTest(ele, `Test ${testData.tag} element with ${type} type`);
      });
    } else {
      runTest(document.createElement(testData.tag), `Test ${testData.tag} element`);
    }
  });
</script>
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
  "source_name": "html/semantics/forms/constraints/form-validation-willValidate-datalist.html"
}
```
