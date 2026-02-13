# html/semantics/forms/the-input-element/month.html

Counts:
- errors: 0
- warnings: 7
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-input-element/month.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
  <head>
    <title>Inputs Month</title>
    <link rel="author" title="Morishita Hiromitsu" href="mailto:hero@asterisk-works.jp">
    <link rel="author" title="kaseijin" href="mailto:pcmkas@gmail.com">
    <link rel="author" title="Denis Ah-Kang" href="mailto:denis@w3.org">
    <link rel="help" href="https://html.spec.whatwg.org/multipage/#months">
    <link rel="help" href="https://html.spec.whatwg.org/multipage/#month-state-(type=month)">
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
  </head>
  <body>
    <h1>Inputs Month</h1>
    <div style="display: none">
      <input id="valid_value_1" type="month" value="20133-12" />
      <input id="valid_value_2" type="month" value="2013-12" />
      <input id="valid_value_3" type="month" value="0003-01" />
      <input id="valid" type="month" value="2011-11" min="2011-01" max="2011-12" />
      <input id="invalid_value" type="month" value="invalid-month" min="2011-01" max="2011-12"/>
      <input id="value_can_be_empty_string" type="month" value="2013-06" />
      <input id="invalid_value_with_two_digits_year" type="month" value="13-06" />
      <input id="invalid_value_is_set" type="month" />
      <input id="step_attribute_is_invalid_value" type="month" value="2013-06" step="invalid_step_value" />
      <input id="invalid_month_too_high" type="month" value="2013-13" />
      <input id="invalid_month_too_low" type="month" value="2013-00" />
      <input id="invalid_year_all_zero" type="month" value="0000-10" />
      <input id="invalid_month_with_one_number" type="month" value="2013-1" />
      <input id="invalid_month_non_numerical" type="month" value="2013-abc" />
      <input id="invalid_date_additional_tuples" type="month" value="2013-11-1-1" />
    </div>

    <div id="log"></div>

    <script>
      test(function() {
        assert_equals(document.getElementById("valid_value_1").value, "20133-12")
      }, "year can be more than four digits");

      test(function() {
        assert_equals(document.getElementById("valid_value_2").value, "2013-12")
      }, "valid value test");

      test(function() {
        assert_equals(document.getElementById("valid_value_3").value, "0003-01")
      }, "year can contain prefixes of zero, as long as there are at least four digits");

      test(function() {
        assert_equals(document.getElementById("valid").type, "month")
      }, "month type support on input element");

      test(function() {
        assert_equals(document.getElementById("invalid_value").value, "")
      }, "User agents must not allow the user to set the value to a non-empty string that is not a valid month string.");

      test(function() {
        document.getElementById("value_can_be_empty_string").value = "";
        assert_equals(document.getElementById("value_can_be_empty_string").value, "")
      }, "Month value can be empty string.");

      test(function() {
        assert_equals(document.getElementById("invalid_value_with_two_digits_year").value, "")
      }, "When value attribute has two digits year value, the value,which is invalid, must return empty string.");

      test(function() {
        document.getElementById("invalid_value_is_set").value = "invalid value";
        assert_equals(document.getElementById("invalid_value_is_set").value, "")
      }, "When value is set with invalid value, the value must return empty string.");

      test(function() {
        document.getElementById("step_attribute_is_invalid_value").stepUp();
        assert_equals(document.getElementById("step_attribute_is_invalid_value").value, "2013-07")
      }, "When step attribute is given invalid value, it must ignore the invalid value and use defaul value instead.");

      test(function() {
        assert_equals(document.getElementById("invalid_month_too_high").value, "");
      }, "Month should be <= 13: If the value of the element is not a valid month string, then set it to the empty string instead.");

      test(function() {
        assert_equals(document.getElementById("invalid_month_too_low").value, "");
      }, "Month should be > 0: If the value of the element is not a valid month string, then set it to the empty string instead.>");

      test(function() {
        assert_equals(document.getElementById("invalid_year_all_zero").value, "");
      }, "Year should be > 0: If the value of the element is not a valid year string, then set it to the empty string instead.>");

      test(function() {
        assert_equals(document.getElementById("invalid_month_with_one_number").value, "");
      }, "Month should be two digits: If the value of the element is not a valid month string, then set it to the empty string instead.>");

      test(function() {
        assert_equals(document.getElementById("invalid_month_non_numerical").value, "");
      }, "Month should be two digits not characters: If the value of the element is not a valid month string, then set it to the empty string instead.>");

      test(function() {
        assert_equals(document.getElementById("invalid_date_additional_tuples").value, "");
      }, "Value should be two parts: If the value of the element is not a valid month string, then set it to the empty string instead.>");
    </script>
  </body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.input.month.invalid",
      "message": "Bad value “invalid-month” for attribute “value” on element “input”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1032,
        "byte_start": 942,
        "col": 7,
        "line": 20
      }
    },
    {
      "category": "Html",
      "code": "html.input.step.invalid",
      "message": "Bad value “invalid_step_value” for attribute “step” on element “input”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1354,
        "byte_start": 1253,
        "col": 7,
        "line": 24
      }
    },
    {
      "category": "Html",
      "code": "html.input.month.invalid",
      "message": "Bad value “2013-13” for attribute “value” on element “input”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1427,
        "byte_start": 1361,
        "col": 7,
        "line": 25
      }
    },
    {
      "category": "Html",
      "code": "html.input.month.invalid",
      "message": "Bad value “2013-00” for attribute “value” on element “input”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1499,
        "byte_start": 1434,
        "col": 7,
        "line": 26
      }
    },
    {
      "category": "Html",
      "code": "html.input.month.invalid",
      "message": "Bad value “2013-abc” for attribute “value” on element “input”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1729,
        "byte_start": 1657,
        "col": 7,
        "line": 29
      }
    },
    {
      "category": "Html",
      "code": "html.input.month.invalid",
      "message": "Bad value “2013-11-1-1” for attribute “value” on element “input”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1814,
        "byte_start": 1736,
        "col": 7,
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
  "source_name": "html/semantics/forms/the-input-element/month.html"
}
```
