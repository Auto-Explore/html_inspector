# html/semantics/forms/constraints/form-validation-validity-valueMissing-weekmonth.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/constraints/form-validation-validity-valueMissing-weekmonth.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html

<!DOCTYPE html>
<meta charset="utf-8">
<title>The constraint validation API Test: element.validity.valueMissing</title>
<link rel="author" title="Intel" href="http://www.intel.com/">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#dom-validitystate-valuemissing">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#the-constraint-validation-api">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="support/validator.js"></script>
<div id="log"></div>
<form>
  <input id="messagetest" type="checkbox" required="" disabled="">
</form>

<script>
  var testElements = [
    {
      tag: "input",
      types: ["month"],
      testData: [
        {conditions: {required: false, value: ""}, expected: false, name: "[target] The required attribute is not set"},
        {conditions: {required: true, value: "2000-12"}, expected: false, name: "[target] Valid month string(2000-12)"},
        {conditions: {required: true, value: "9999-01"}, expected: false, name: "[target] Valid month string(9999-01)"},
        {conditions: {required: true, value: 1234567}, expected: true, expectedImmutable: false, name: "[target] The value attribute is a number(1234567)"},
        {conditions: {required: true, value: new Date()}, expected: true, expectedImmutable: false, name: "[target] The value attribute is a Date object"},
        {conditions: {required: true, value: "2000-99"}, expected: true, expectedImmutable: false, name: "[target] Invalid month string(2000-99)"},
        {conditions: {required: true, value: "37-01"}, expected: true, expectedImmutable: false, name: "[target] Invalid month string(37-01)"},
        {conditions: {required: true, value: "2000/01"}, expected: true, expectedImmutable: false, name: "[target] Invalid month string(2000/01)"},
        {conditions: {required: true, value: ""}, expected: true, expectedImmutable: false, name: "[target] The value attribute is empty string"}
      ]
    },
    {
      tag: "input",
      types: ["week"],
      testData: [
        {conditions: {required: false, value: ""}, expected: false, name: "[target] The required attribute is not set"},
        {conditions: {required: true, value: "2000-W12"}, expected: false, name: "[target] Valid week string(2000-W12)"},
        {conditions: {required: true, value: "9999-W01"}, expected: false, name: "[target] Valid week string(9999-W01)"},
        {conditions: {required: true, value: 1234567}, expected: true, expectedImmutable: false, name: "[target] The value attribute is a number(1234567)"},
        {conditions: {required: true, value: new Date()}, expected: true, expectedImmutable: false, name: "[target] The value attribute is a Date object"},
        {conditions: {required: true, value: "2000-W99"}, expected: true, expectedImmutable: false, name: "[target] Invalid week string(2000-W99)"},
        {conditions: {required: true, value: "2000-W00"}, expected: true, expectedImmutable: false, name: "[target] invalid week string(2000-W00)"},
        {conditions: {required: true, value: "2000-w01"}, expected: true, expectedImmutable: false, name: "[target] invalid week string(2000-w01)"},
        {conditions: {required: true, value: ""}, expected: true, expectedImmutable: false, name: "[target] The value attribute is empty string"}
      ]
    }
  ];

  validator.run_test(testElements, "valueMissing");

  test(() => {
    assert_equals(document.getElementById("messagetest").validationMessage, '');
  }, 'validationMessage should return empty string when willValidate is false and valueMissing is true');
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
  "source_name": "html/semantics/forms/constraints/form-validation-validity-valueMissing-weekmonth.html"
}
```
