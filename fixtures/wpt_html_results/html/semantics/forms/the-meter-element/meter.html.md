# html/semantics/forms/the-meter-element/meter.html

Counts:
- errors: 0
- warnings: 25
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-meter-element/meter.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
  <head>
    <title>The meter element</title>
    <link rel="author" title="Tomoyuki SHIMIZU" href="mailto:tomoyuki.labs@gmail.com">
    <link rel="author" title="Denis Ah-Kang" href="mailto:denis@w3.org">
    <link rel="help" href="https://html.spec.whatwg.org/multipage/#the-meter-element">
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
  </head>
  <body>
    <h1>Meter Element</h1>
    <div id="log"></div>
    <div style="display: none;">
      <meter id="meter_illegal_value" value="abc"></meter>
      <meter id="meter_without_min" value="-10"></meter>
      <meter id="meter_without_max" value="10"></meter>
      <meter id="meter_min_without_max_1" value="10" min="-3.1"></meter>
      <meter id="meter_min_without_max_2" value="210" min="12.1"></meter>
      <meter id="meter_max_without_min_1" value="-10" max="-5342.55"></meter>
      <meter id="meter_max_without_min_2" value="210" max="-9.9"></meter>
      <meter id="meter_illegal_min" value="-2" min="hugfe"></meter>
      <meter id="meter_illegal_max" value="2.4" max="min"></meter>
      <meter id="meter_illegal_low_with_min" value="-20"  min="-10.3" low="ahuge"></meter>
      <meter id="meter_illegal_high_with_max" value="2.4" high="old" max="1.5"></meter>
      <meter id="meter_smaller_than_min" value="-10" min="4.5"></meter>
      <meter id="meter_larger_than_max" value="2345.53" max="52.02"></meter>
      <meter id="meter_default_low_and_high_1" value="40" min="-12.3" max="3.4"></meter>
      <meter id="meter_default_low_and_high_2" value="23"></meter>
      <meter id="meter_low_smaller_than_min" value="-4" min="12.3" low="34"></meter>
      <meter id="meter_low_larger_than_max" value="-1" min="-50" low="-5" max="-34.5"></meter>
      <meter id="meter_high_smaller_than_min" value="-4" min="12.3" high="34"></meter>
      <meter id="meter_high_larger_than_max" value="-1" min="-50" high="-5" max="-34.5"></meter>
      <meter id="meter_high_smaller_than_low" value="-9" min="-20" low="-10.3" high="-15.2" max="-2"></meter>
      <meter id="meter_low_without_min" value="-1" low="-5"></meter>
      <meter id="meter_high_without_max" value="50000" high="4"></meter>
      <meter id="meter_optimum_smaller_than_min" value="-8" optimum="-4"></meter>
      <meter id="meter_optimum_larger_than_max" value="324" optimum="4.6"></meter>
      <meter id="meter_default_optimum" value="10" min="-132.35" max="33.423"></meter>
    </div>
    <script>
      var meters = [
        {value: 0, expectedValue: 0, expectedMin: 0, expectedMax: 1.0, expectedLow: 0, expectedHigh: 1.0, expectedOptimum: 0.5, testname: "Default values"},
        {value: 3, expectedValue: 3, min: -10.1, expectedMin: -10.1, max: 10.1, expectedMax: 10.1, low: -9.1, expectedLow: -9.1, high: 9.1, expectedHigh: 9.1, optimum: 3, expectedOptimum: 3, testname: "Setting values to min, max, low, high and optimum"},
        {value: 0, expectedValue: 0, min: 0, expectedMin: 0, max: -1.0, expectedMax: 0, expectedLow: 0, expectedHigh: 0, expectedOptimum: 0, testname: "max < min"},
        {value: 0, expectedValue: 10, min: 10, expectedMin: 10, max: 20, expectedMax: 20, expectedLow: 10, expectedHigh: 20, expectedOptimum: 15, testname: "value < min"},
        {value: 30, expectedValue: 20, min: 10, expectedMin: 10, max: 20, expectedMax: 20, expectedLow: 10, expectedHigh: 20, expectedOptimum: 15, testname: "value > max"},
        {value: 15, expectedValue: 15, min: 10, expectedMin: 10, max: 20, expectedMax: 20, low: 5, expectedLow: 10, expectedHigh: 20, expectedOptimum: 15, testname: "low < min"},
        {value: 15, expectedValue: 15, min: 10, expectedMin: 10, max: 20, expectedMax: 20, low: 25, expectedLow: 20, expectedHigh: 20, expectedOptimum: 15, testname: "low > max"},
        {value: 15, expectedValue: 15, min: 10, expectedMin: 10, max: 20, expectedMax: 20, low: 12, expectedLow: 12, high: 10, expectedHigh: 12, expectedOptimum: 15, testname: "high < low"},
        {value: 15, expectedValue: 15, min: 10, expectedMin: 10, max: 20, expectedMax: 20, low: 10, expectedLow: 10, high: 22, expectedHigh: 20, expectedOptimum: 15, testname: "high > max"},
        {value: 15, expectedValue: 15, min: 10, expectedMin: 10, max: 20, expectedMax: 20, expectedLow: 10, expectedHigh: 20, optimum: 9, expectedOptimum: 10, testname: "optimum < min"},
        {value: 15, expectedValue: 15, min: 10, expectedMin: 10, max: 20, expectedMax: 20, expectedLow: 10, expectedHigh: 20, optimum: 21, expectedOptimum: 20, testname: "optimum > max"}
      ];
      for (var i = 0; i < meters.length; i++) {
        var m = meters[i];
        test(function() {
          var meter = document.createElement("meter");
          meter.value = m.value;
          if (m.min) meter.min= m.min;
          if (m.max) meter.max = m.max;
          if (m.low) meter.low = m.low;
          if (m.high) meter.high = m.high;
          if (m.optimum) meter.optimum = m.optimum;
          assert_equals(meter.value, m.expectedValue, "meter value");
          assert_equals(meter.min, m.expectedMin, "min value");
          assert_equals(meter.max, m.expectedMax, "max value");
          assert_equals(meter.low, m.expectedLow, "low value");
          assert_equals(meter.high, m.expectedHigh, "high value");
          assert_equals(meter.optimum, m.expectedOptimum, "optimum value");
        }, m.testname);
      }
      test(function() {
          var meter = document.createElement("meter");
          assert_throws_js(TypeError, function() { meter.value = "foobar"; }, "value attribute");
          assert_throws_js(TypeError, function() { meter.min = "foobar"; }, "min attribute");
          assert_throws_js(TypeError, function() { meter.max = "foobar"; }, "max attribute");
          assert_throws_js(TypeError, function() { meter.low = "foobar"; }, "low attribute");
          assert_throws_js(TypeError, function() { meter.high = "foobar"; }, "high attribute");
          assert_throws_js(TypeError, function() { meter.optimum = "foobar"; }, "optimum attribute");
      }, "Invalid floating-point number values");

    </script>
    <script type="text/javascript">
      test(function() {
          assert_equals(document.getElementById('meter_illegal_value').value, 0);
        }, "value must be 0 when a string is given");

      test(function() {
          assert_equals(document.getElementById('meter_without_min').min, 0);
        }, "default value of min is 0");

      test(function() {
          assert_equals(document.getElementById('meter_without_min').value, 0);
        }, "If min is not specified and value is smaller than the default value of min (i.e. 0), the actual value must be 0");

      test(function() {
          assert_equals(document.getElementById('meter_without_max').max, 1.0);
        }, "default value of max is 1.0");

      test(function() {
          assert_equals(document.getElementById('meter_without_max').value, 1.0);
        }, "If max is not specified and value is larger than the default value of max (i.e. 1.0), the actual value must be 1.0");

      test(function() {
          assert_equals(document.getElementById('meter_min_without_max_1').max, 1.0);
        }, "If a value smaller than 1.0 is given to min and max is not specified, max must be the same value as its default value (i.e. 1.0)");

      test(function() {
          assert_equals(document.getElementById('meter_min_without_max_1').value, 1.0);
        }, "If a value smaller than 1.0 is given to min, max is not specified, and value is larger than the default value of max (i.e. 1.0), the actual value must be 1.0");

      test(function() {
          assert_equals(document.getElementById('meter_min_without_max_2').max, 12.1);
        }, "If a value larger than or equal to 1.0 is given to min and max is not specified, max must be the same value as min");

      test(function() {
          assert_equals(document.getElementById('meter_min_without_max_2').value, 12.1);
        }, "If a value larger than or equal to 1.0 is given to min and max is not specified, the actual value must be the same value as min");

      test(function() {
          assert_equals(document.getElementById('meter_max_without_min_1').min, 0);
        }, "If a value smaller than 0 is given to max and min is not specified, min must be be the same value as its default value (i.e. 0)");

      test(function() {
          assert_equals(document.getElementById('meter_max_without_min_1').max, 0);
        }, "If a value smaller than 0 is given to max and min is not specified, max must be be the same value as the default value of min (i.e. 0)");

      test(function() {
          assert_equals(document.getElementById('meter_max_without_min_1').value, 0);
        }, "If a value smaller than 0 is given to max and min is not specified, the actual value must be be the same value as the default value of min (i.e. 0)");

      test(function() {
          assert_equals(document.getElementById('meter_max_without_min_2').max, 0);
        }, "If a value larger than or equal to 0 is given to max and min is not specified, max must be the same value as the default value of min (i.e. 0)");

      test(function() {
          assert_equals(document.getElementById('meter_max_without_min_2').min, 0);
        }, "If a value larger than or equal to 0 is given to max and min is not specified, min must be the same value as its default value (i.e. 0)");

      test(function() {
          assert_equals(document.getElementById('meter_max_without_min_2').value, 0);
        }, "If a value larger than or equal to 0 is given to max and min is not specified, the actual value must be the same value as the default value of min (i.e. 0)");

      test(function() {
          assert_equals(document.getElementById('meter_illegal_min').min, 0);
        }, "min must be 0 when a string is given");

      test(function() {
          assert_equals(document.getElementById('meter_illegal_min').value, 0);
        }, "If a string is given to min and value is smaller than the default value of min (i.e. 0), the actual value must be 0");

      test(function() {
          assert_equals(document.getElementById('meter_illegal_max').max, 1.0);
        }, "max must be 1.0 when a string is given");

      test(function() {
          assert_equals(document.getElementById('meter_illegal_max').value, 1.0);
        }, "If a string is given to max and value is larger than the default value of min (i.e. 1.0), the actual value must be 1.0");

      test(function() {
          assert_equals(document.getElementById('meter_illegal_low_with_min').low, -10.3);
        }, "giving a string to low must not affect the actual value");

      test(function() {
          assert_equals(document.getElementById('meter_illegal_high_with_max').high, 1.5);
        }, "high must equal max when a string is given to high");

      test(function() {
          assert_equals(document.getElementById('meter_illegal_high_with_max').value, 1.5);
        }, "giving a string to high must not affect the actual value");

      test(function() {
          assert_equals(document.getElementById('meter_smaller_than_min').value, 4.5);
        }, "value must not be smaller than min");

      test(function() {
          assert_equals(document.getElementById('meter_larger_than_max').value, 52.02);
        }, "value must not be larger than max");

      test(function() {
          var e = document.getElementById('meter_default_low_and_high_1');
          assert_array_equals([e.low,e.high], [-12.3,3.4]);
        }, "default low and high values equal min and max, respectively");

      test(function() {
          var e = document.getElementById('meter_default_low_and_high_2');
          assert_array_equals([e.low,e.high], [0,1.0]);
        }, "default low and high values equal 0 and 1.0 respectively, if both low and high are not specified");

      test(function() {
          var e = document.getElementById('meter_low_smaller_than_min');
          assert_array_equals([e.low,e.min,e.value], [12.3,12.3,12.3]);
        }, "low must not be smaller than min");

      test(function() {
          var e = document.getElementById('meter_low_larger_than_max');
          assert_array_equals([e.low,e.max,e.value], [-34.5,-34.5,-34.5]);
        }, "low must not be larger than max");

      test(function() {
          var e = document.getElementById('meter_high_smaller_than_min');
          assert_array_equals([e.high,e.min,e.value], [12.3,12.3,12.3]);
        }, "high must not be smaller than min");

      test(function() {
          var e = document.getElementById('meter_high_larger_than_max');
          assert_array_equals([e.high,e.max,e.value], [-34.5,-34.5,-34.5]);
        }, "high must not be larger than max");

      test(function() {
          var e = document.getElementById('meter_low_without_min');
          assert_array_equals([e.low,e.min,e.value], [0,0,0]);
        }, "If min is not specified, low must not be smaller than default value of min (i.e. 0)");

      test(function() {
          var e = document.getElementById('meter_high_smaller_than_low');
          assert_array_equals([e.low,e.high,e.value], [-10.3,-10.3,-9]);
        }, "If a value smaller than low is given to high, it must be set to the same value as low");

      test(function() {
          var e = document.getElementById('meter_high_without_max');
          assert_array_equals([e.high,e.value], [1.0,1.0]);
        }, "If max is not specified, high must not be larger than default value of max (i.e. 1.0)");

      test(function() {
          assert_equals(document.getElementById('meter_optimum_smaller_than_min').optimum, 0);
        }, "optimum smaller than min");

      test(function() {
          var e = document.getElementById('meter_optimum_smaller_than_min');
          assert_array_equals([e.min,e.value], [0,0]);
        }, "optimum (smaller than min) must not affect min and the actual value");

      test(function() {
          assert_equals(document.getElementById('meter_optimum_larger_than_max').optimum, 1.0);
        }, "optimum smaller than max");

      test(function() {
          var e = document.getElementById('meter_optimum_larger_than_max');
          assert_array_equals([e.max,e.value], [1.0,1.0]);
        }, "optimum (larger than max) must not affect max and the actual value");

      test(function() {
          var e = document.getElementById('meter_default_optimum');
          assert_equals(e.optimum, (e.max + e.min) / 2);
        }, "default optimum value is the midpoint between min and max");
    </script>
  </body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.meter.value.ge_zero_when_min_absent",
      "message": "The value of the “value” attribute must be greater than or equal to zero when the “min” attribute is absent.",
      "severity": "Warning",
      "span": {
        "byte_end": 641,
        "byte_start": 599,
        "col": 7,
        "line": 16
      }
    },
    {
      "category": "Html",
      "code": "html.meter.value.le_one_when_max_absent",
      "message": "The value of the “value” attribute must be less than or equal to one when the “max” attribute is absent.",
      "severity": "Warning",
      "span": {
        "byte_end": 697,
        "byte_start": 656,
        "col": 7,
        "line": 17
      }
    },
    {
      "category": "Html",
      "code": "html.meter.value.le_one_when_max_absent",
      "message": "The value of the “value” attribute must be less than or equal to one when the “max” attribute is absent.",
      "severity": "Warning",
      "span": {
        "byte_end": 770,
        "byte_start": 712,
        "col": 7,
        "line": 18
      }
    },
    {
      "category": "Html",
      "code": "html.meter.value.le_one_when_max_absent",
      "message": "The value of the “value” attribute must be less than or equal to one when the “max” attribute is absent.",
      "severity": "Warning",
      "span": {
        "byte_end": 844,
        "byte_start": 785,
        "col": 7,
        "line": 19
      }
    },
    {
      "category": "Html",
      "code": "html.meter.value.ge_zero_when_min_absent",
      "message": "The value of the “value” attribute must be greater than or equal to zero when the “min” attribute is absent.",
      "severity": "Warning",
      "span": {
        "byte_end": 922,
        "byte_start": 859,
        "col": 7,
        "line": 20
      }
    },
    {
      "category": "Html",
      "code": "html.meter.value_le_max",
      "message": "The value of the “value” attribute must be less than or equal to the value of the “max” attribute.",
      "severity": "Warning",
      "span": {
        "byte_end": 996,
        "byte_start": 937,
        "col": 7,
        "line": 21
      }
    },
    {
      "category": "Html",
      "code": "html.meter.value.ge_zero_when_min_absent",
      "message": "The value of the “value” attribute must be greater than or equal to zero when the “min” attribute is absent.",
      "severity": "Warning",
      "span": {
        "byte_end": 1064,
        "byte_start": 1011,
        "col": 7,
        "line": 22
      }
    },
    {
      "category": "Html",
      "code": "html.meter.value.le_one_when_max_absent",
      "message": "The value of the “value” attribute must be less than or equal to one when the “max” attribute is absent.",
      "severity": "Warning",
      "span": {
        "byte_end": 1131,
        "byte_start": 1079,
        "col": 7,
        "line": 23
      }
    },
    {
      "category": "Html",
      "code": "html.meter.min_le_value",
      "message": "The value of the “min” attribute must be less than or equal to the value of the “value” attribute.",
      "severity": "Warning",
      "span": {
        "byte_end": 1222,
        "byte_start": 1146,
        "col": 7,
        "line": 24
      }
    },
    {
      "category": "Html",
      "code": "html.meter.value_le_max",
      "message": "The value of the “value” attribute must be less than or equal to the value of the “max” attribute.",
      "severity": "Warning",
      "span": {
        "byte_end": 1310,
        "byte_start": 1237,
        "col": 7,
        "line": 25
      }
    },
    {
      "category": "Html",
      "code": "html.meter.min_le_value",
      "message": "The value of the “min” attribute must be less than or equal to the value of the “value” attribute.",
      "severity": "Warning",
      "span": {
        "byte_end": 1382,
        "byte_start": 1325,
        "col": 7,
        "line": 26
      }
    },
    {
      "category": "Html",
      "code": "html.meter.value_le_max",
      "message": "The value of the “value” attribute must be less than or equal to the value of the “max” attribute.",
      "severity": "Warning",
      "span": {
        "byte_end": 1459,
        "byte_start": 1397,
        "col": 7,
        "line": 27
      }
    },
    {
      "category": "Html",
      "code": "html.meter.value_le_max",
      "message": "The value of the “value” attribute must be less than or equal to the value of the “max” attribute.",
      "severity": "Warning",
      "span": {
        "byte_end": 1548,
        "byte_start": 1474,
        "col": 7,
        "line": 28
      }
    },
    {
      "category": "Html",
      "code": "html.meter.value.le_one_when_max_absent",
      "message": "The value of the “value” attribute must be less than or equal to one when the “max” attribute is absent.",
      "severity": "Warning",
      "span": {
        "byte_end": 1615,
        "byte_start": 1563,
        "col": 7,
        "line": 29
      }
    },
    {
      "category": "Html",
      "code": "html.meter.min_le_value",
      "message": "The value of the “min” attribute must be less than or equal to the value of the “value” attribute.",
      "severity": "Warning",
      "span": {
        "byte_end": 1700,
        "byte_start": 1630,
        "col": 7,
        "line": 30
      }
    },
    {
      "category": "Html",
      "code": "html.meter.value_le_max",
      "message": "The value of the “value” attribute must be less than or equal to the value of the “max” attribute.",
      "severity": "Warning",
      "span": {
        "byte_end": 1795,
        "byte_start": 1715,
        "col": 7,
        "line": 31
      }
    },
    {
      "category": "Html",
      "code": "html.meter.min_le_value",
      "message": "The value of the “min” attribute must be less than or equal to the value of the “value” attribute.",
      "severity": "Warning",
      "span": {
        "byte_end": 1882,
        "byte_start": 1810,
        "col": 7,
        "line": 32
      }
    },
    {
      "category": "Html",
      "code": "html.meter.value_le_max",
      "message": "The value of the “value” attribute must be less than or equal to the value of the “max” attribute.",
      "severity": "Warning",
      "span": {
        "byte_end": 1979,
        "byte_start": 1897,
        "col": 7,
        "line": 33
      }
    },
    {
      "category": "Html",
      "code": "html.meter.low_le_high",
      "message": "The value of the “low” attribute must be less than or equal to the value of the “high” attribute.",
      "severity": "Warning",
      "span": {
        "byte_end": 2089,
        "byte_start": 1994,
        "col": 7,
        "line": 34
      }
    },
    {
      "category": "Html",
      "code": "html.meter.value.ge_zero_when_min_absent",
      "message": "The value of the “value” attribute must be greater than or equal to zero when the “min” attribute is absent.",
      "severity": "Warning",
      "span": {
        "byte_end": 2158,
        "byte_start": 2104,
        "col": 7,
        "line": 35
      }
    },
    {
      "category": "Html",
      "code": "html.meter.value.le_one_when_max_absent",
      "message": "The value of the “value” attribute must be less than or equal to one when the “max” attribute is absent.",
      "severity": "Warning",
      "span": {
        "byte_end": 2231,
        "byte_start": 2173,
        "col": 7,
        "line": 36
      }
    },
    {
      "category": "Html",
      "code": "html.meter.value.ge_zero_when_min_absent",
      "message": "The value of the “value” attribute must be greater than or equal to zero when the “min” attribute is absent.",
      "severity": "Warning",
      "span": {
        "byte_end": 2313,
        "byte_start": 2246,
        "col": 7,
        "line": 37
      }
    },
    {
      "category": "Html",
      "code": "html.meter.value.le_one_when_max_absent",
      "message": "The value of the “value” attribute must be less than or equal to one when the “max” attribute is absent.",
      "severity": "Warning",
      "span": {
        "byte_end": 2396,
        "byte_start": 2328,
        "col": 7,
        "line": 38
      }
    },
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 6180,
        "byte_start": 6149,
        "col": 5,
        "line": 84
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
  "source_name": "html/semantics/forms/the-meter-element/meter.html"
}
```
