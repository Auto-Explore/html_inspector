# html/semantics/forms/constraints/form-validation-validity-valid-weekmonth.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/constraints/form-validation-validity-valid-weekmonth.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>The constraint validation API Test: element.validity.valid</title>
<link rel="author" title="Intel" href="http://www.intel.com/">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#dom-validitystate-valid">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#the-constraint-validation-api">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="support/validator.js"></script>
<div id="log"></div>
<script>
  var testElements = [
    {
      tag: "input",
      types: ["month"],
      testData: [
        {conditions: {max: "2000-01", value: "2001-01"}, expected: false, name: "[target] validity.valid must be false if validity.rangeOverflow is true"},
        {conditions: {min: "2001-01", value: "2000-01"}, expected: false, name: "[target] validity.valid must be false if validity.rangeUnderflow is true"},
        // Step checks that "months since Jan 1970" is evenly divisible by `step`
        {conditions: {step: 3, value: "2001-02"}, expected: false, name: "[target] validity.valid must be false if validity.stepMismatch is true"},
        {conditions: {required: true, value: ""}, expected: false, expectedImmutable: true, name: "[target] validity.valid must be false if validity.valueMissing is true"}
      ]
    },
    {
      tag: "input",
      types: ["week"],
      testData: [
        {conditions: {max: "2000-W01", value: "2001-W01"}, expected: false, name: "[target] validity.valid must be false if validity.rangeOverflow is true"},
        {conditions: {min: "2001-W01", value: "2000-W01"}, expected: false, name: "[target] validity.valid must be false if validity.rangeUnderflow is true"},
        {conditions: {step: 2 * 1 * 604800000, value: "2001-W03"}, expected: false, name: "[target] validity.valid must be false if validity.stepMismatch is true"},
        {conditions: {required: true, value: ""}, expected: false, expectedImmutable: true, name: "[target] validity.valid must be false if validity.valueMissing is true"}
      ]
    }
  ];

  validator.run_test(testElements, "isValid");
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
  "source_name": "html/semantics/forms/constraints/form-validation-validity-valid-weekmonth.html"
}
```
