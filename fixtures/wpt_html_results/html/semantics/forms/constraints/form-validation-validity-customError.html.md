# html/semantics/forms/constraints/form-validation-validity-customError.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/constraints/form-validation-validity-customError.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>The constraint validation API Test: element.validity.customError</title>
<link rel="author" title="Intel" href="http://www.intel.com/">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#dom-validitystate-customerror">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#the-constraint-validation-api">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="support/validator.js"></script>
<div id="log"></div>
<script>
var testElements = [
    {
      tag: "input",
      types: [],
      testData: [
        {conditions: {message: "My custom error"}, expected: true, name: "[target] The validity.customError must be true if the custom validity error message is not empty"},
        {conditions: {message: ""}, expected: false, name: "[target] The validity.customError must be false if the custom validity error message is empty"}
      ]
    },
    {
      tag: "button",
      types: [],
      testData: [
        {conditions: {message: "My custom error"}, expected: true, name: "[target] The validity.customError must be true if the custom validity error message is not empty"},
        {conditions: {message: ""}, expected: false, name: "[target] The validity.customError must be false if the custom validity error message is empty"}
      ]
    },
    {
      tag: "select",
      types: [],
      testData: [
        {conditions: {message: "My custom error"}, expected: true, name: "[target] The validity.customError must be true if the custom validity error message is not empty"},
        {conditions: {message: ""}, expected: false, name: "[target] The validity.customError must be false if the custom validity error message is empty"}
      ]
    },
    {
      tag: "textarea",
      types: [],
      testData: [
        {conditions: {message: "My custom error"}, expected: true, name: "[target] The validity.customError must be true if the custom validity error message is not empty"},
        {conditions: {message: ""}, expected: false, name: "[target] The validity.customError must be false if the custom validity error message is empty"}
      ]
    }
  ]

  validator.run_test(testElements, "customError");
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
  "source_name": "html/semantics/forms/constraints/form-validation-validity-customError.html"
}
```
