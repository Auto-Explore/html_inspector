# html/semantics/forms/the-input-element/week.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-input-element/week.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<title>Form input type=week</title>
<link rel="author" title="Denis Ah-Kang" href="mailto:denis@w3.org">
<link rel=help href="https://html.spec.whatwg.org/multipage/#weeks">
<link rel=help href="https://html.spec.whatwg.org/multipage/#week-state-(type=week)">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<script>
  var weeks = [
    {value: "", expected: "", testname: "empty value"},
    {value: "2014-W52", expected: "2014-W52", testname: "Valid value: Value should be 2014-W52"},
    {value: "2014-W53", expected: "", testname: "2014 has 52 weeks: Value should be empty"},
    {value: "2015-W53", expected: "2015-W53", testname: "2015 has 53 weeks: Value should be 2015-W53"},
    {value: "2014", expected: "", testname: "Invalid value: year only"},
    {value: "2014W", expected: "", testname: "Invalid value: no week number"},
    {value: "2014W52", expected: "", testname: "Invalid value: no '-' (U+002D)"},
    {value: "-W52", expected: "", testname: "Invalid value: yearless week"},
    {value: "2017-w52", expected: "", testname: "Invalid value: should be capital letter 'W'"},
    {value: "2017-W52-", expected: "", testname: "Invalid value: incorrect with '-' at the end"},
    {value: "2017-W52-12", expected: "", testname: "Invalid value: value should be two parts"},
    {value: "W52", expected: "", testname: "Invalid value: yearless week and no '-' (U+002D)"},
    {value: "2014-W03", attributes: { min: "2014-W02" }, expected: "2014-W03", testname: "Value >= min attribute"},
    {value: "2014-W01", attributes: { min: "2014-W02" }, expected: "2014-W01", testname: "Value < min attribute"},
    {value: "2014-W10", attributes: { max: "2014-W11" }, expected: "2014-W10", testname: "Value <= max attribute"},
    {value: "2014-W12", attributes: { max: "2014-W11" }, expected: "2014-W12", testname: "Value > max attribute"}
  ];
  for (var i = 0; i < weeks.length; i++) {
    var w = weeks[i];
    test(function() {
      var input = document.createElement("input");
      input.type = "week";
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
  "source_name": "html/semantics/forms/the-input-element/week.html"
}
```
