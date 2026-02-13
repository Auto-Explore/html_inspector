# html/semantics/forms/the-input-element/time-2.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-input-element/time-2.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<title>Form input type=time</title>
<link rel="author" title="Denis Ah-Kang" href="mailto:denis@w3.org">
<link rel=help href="https://html.spec.whatwg.org/multipage/#times">
<link rel=help href="https://html.spec.whatwg.org/multipage/#time-state-(type=time)">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<script>
  var times = [
    {value: "", expected: "", testname: "empty value"},
    {value: "00:00", expected: "00:00", testname: "Valid value: value should be 00:00"},
    {value: "00:00:00", expected: "00:00:00", testname: "Valid value: value should be 00:00:00"},
    {value: "00:00:00.0", expected: "00:00:00.0", testname: "Valid value: value should be 00:00:00.0"},
    {value: "00:00:00.00", expected: "00:00:00.00", testname: "Valid value: value should be 00:00:00.00"},
    {value: "00:00:00.000", expected: "00:00:00.000", testname: "Valid value: value should be 00:00:00.000"},
    {value: "00:00:00.0000", expected: "", testname: "Invalid value: fraction should have one, two or three ASCII digits. Value should be empty"},
    {value: "0:00:00.000", expected: "", testname: "Invalid value: hour should have two ASCII digits. Value should be empty"},
    {value: "00:0:00.000", expected: "", testname: "Invalid value: minutes should have two ASCII digits. Value should be empty"},
    {value: "00:00:0.000", expected: "", testname: "Invalid value: seconds should have two ASCII digits. Value should be empty"},
    {value: "24:00:00.000", expected: "", testname: "Invalid value: hour > 23. Value should be empty"},
    {value: "00:60:00.000", expected: "", testname: "Invalid value: minute > 59. Value should be empty"},
    {value: "00:00:60.000", expected: "", testname: "Invalid value: second > 59. Value should be empty"},
    {value: "12:00:00.001", attributes: { min: "12:00:00.000" }, expected: "12:00:00.001", testname: "Value >= min attribute"},
    {value: "12:00:00.000", attributes: { min: "12:00:00.001" }, expected: "12:00:00.000", testname: "Value < min attribute"},
    {value: "12:00:00.000", attributes: { max: "12:00:00.001" }, expected: "12:00:00.000", testname: "Value <= max attribute"},
    {value: "12:00:00.001", attributes: { max: "12:00:00.000" }, expected: "12:00:00.001", testname: "Value > max attribute"}
  ];
  for (var i = 0; i < times.length; i++) {
    var w = times[i];
    test(function() {
      var input = document.createElement("input");
      input.type = "time";
      input.value = w.value;
      for(var attr in w.attributes) {
        input[attr] = w.attributes[attr];
      }
      assert_equals(input.value, w.expected);
    }, w.testname);
  }
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
  "source_name": "html/semantics/forms/the-input-element/time-2.html"
}
```
