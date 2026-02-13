# html/semantics/forms/the-input-element/datetime-weekmonth.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-input-element/datetime-weekmonth.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
  <head>
    <title>Date and Time Inputs</title>
    <meta name=viewport content="width=device-width, maximum-scale=1.0, user-scalable=no" />
    <link rel="author" title="Fabrice Clari" href="mailto:f.clari@inno-group.com">
    <link rel="author" title="Dimitri Bocquet" href="mailto:Dimitri.Bocquet@mosquito-fp7.eu">
    <link rel="help" href="https://html.spec.whatwg.org/multipage/#the-input-element">
    <link rel="help" href="https://html.spec.whatwg.org/multipage/#dom-input-type">
    <link rel="help" href="https://html.spec.whatwg.org/multipage/#dom-input-value">
    <link rel="help" href="https://html.spec.whatwg.org/multipage/#dom-input-min">
    <link rel="help" href="https://html.spec.whatwg.org/multipage/#dom-input-max">
    <link rel="help" href="https://html.spec.whatwg.org/multipage/#dom-input-step">
    <link rel="help" href="https://html.spec.whatwg.org/multipage/#dom-input-stepup">
    <link rel="help" href="https://html.spec.whatwg.org/multipage/#dom-input-stepdown">
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
  </head>
  <body>

      <h1>Date and Time Inputs</h1>
      <div style="display: none">
        <input type="month" value="2011-01" min="2011-01" max="2011-12" step="2" />
        <input type="week" value="2011-W40" min="2011-W20" max="2011-W50" step="2" />
    </div>

    <div id="log">
    </div>

  <script type="text/javascript">
    test(function() {assert_equals(document.getElementsByTagName("input")[0].type, "month")}, "month type support on input element");
    test(function() {assert_equals(document.getElementsByTagName("input")[0].value, "2011-01")}, "[month] The value must be a value that is a valid global date and time string");
    test(function() {assert_equals(document.getElementsByTagName("input")[0].min, "2011-01")}, "[month] The min attribute must have a value that is a valid global date and time string");
    test(function() {assert_equals(document.getElementsByTagName("input")[0].max, "2011-12")}, "[month] The max attribute must have a value that is a valid global date and time string");
    test(function() {assert_equals(document.getElementsByTagName("input")[0].step, "2")}, "[month] The step attribute must be expressed in seconds");
    test(function() {assert_true(typeof(document.getElementsByTagName("input")[0].stepUp) == "function")}, "[month] stepUp method support on input 'month' element");
    test(function() {assert_true(typeof(document.getElementsByTagName("input")[0].stepDown) == "function")}, "[month] stepDown method support on input 'month' element");

    test(function() {assert_equals(document.getElementsByTagName("input")[1].type, "week")}, "week type support on input element");
    test(function() {assert_equals(document.getElementsByTagName("input")[1].value, "2011-W40")}, "[week] The value must be a value that is a valid global date and time string");
    test(function() {assert_equals(document.getElementsByTagName("input")[1].min, "2011-W20")}, "[week] The min attribute must have a value that is a valid global date and time string");
    test(function() {assert_equals(document.getElementsByTagName("input")[1].max, "2011-W50")}, "[week] The max attribute must have a value that is a valid global date and time string");
    test(function() {assert_equals(document.getElementsByTagName("input")[1].step, "2")}, "[week] The step attribute must be expressed in seconds");
    test(function() {assert_true(typeof(document.getElementsByTagName("input")[1].stepUp) == "function")}, "[week] stepUp method support on input 'week' element");
    test(function() {assert_true(typeof(document.getElementsByTagName("input")[1].stepDown) == "function")}, "[week] stepDown method support on input 'week' element");

  </script>

  </body>

</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.meta.viewport.user_scalable_no",
      "message": "Consider avoiding viewport values that prevent users from resizing documents.",
      "severity": "Warning",
      "span": {
        "byte_end": 164,
        "byte_start": 76,
        "col": 5,
        "line": 5
      }
    },
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 1472,
        "byte_start": 1441,
        "col": 3,
        "line": 30
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/forms/the-input-element/datetime-weekmonth.html"
}
```
