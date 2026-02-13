# html/semantics/forms/the-input-element/input-valueasdate-stepping.html

Counts:
- errors: 1
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-input-element/input-valueasdate-stepping.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<html>
 <head>
  <title>valueAsDate stepping</title>
  <script src="/resources/testharness.js"></script>
  <script src="/resources/testharnessreport.js"></script>
 </head>
 <body>
  <p>
    <h3>input_valueAsDate_stepping</h3>
    <!-- This test verifies that valueAsDate reads and writes Date values,
         that those values step by the correct default step, and that the values
         represent the correct times.
    -->
  </p>

  <hr>

  <div id="log"></div>

  <form method="post"
      enctype="application/x-www-form-urlencoded"
      action=""
      name="input_form">
    <p><input type='date' id='input_date'></p>
    <p><input type='time' id='input_time'></p>
    <p><input type='week' id='input_week'></p>
    <p><input type='month' id='input_month'></p>
  </form>

  <script>
    function test_stepping(inputType, stringValue, steppedString, baseMillis, stepAmount) {
       test(function() {
          // put date in, constructed from a UTC timestamp so the test doesn't
          // vary by local timezone
          input = document.getElementById("input_" + inputType);
          input.valueAsDate = new Date(baseMillis)

          // get string out (using startsWith here to allow for optional
          // seconds and milliseconds)
          var sanitizedStr = input.value;
          assert_true(sanitizedStr.startsWith(stringValue),
             "The input value [" + sanitizedStr + "] must resemble [" + stringValue + "]");

          // get date out
          var sanitized = input.valueAsDate;
          assert_equals(sanitized.getTime(), baseMillis, "The input valueAsDate must represent the same time as the original Date.")

          // step up, get new date out
          input.stepUp()
          var steppedDate = input.valueAsDate;
          assert_equals(steppedDate.getTime(), baseMillis + stepAmount, "Stepping must be by the correct amount")

          // get new string out
          var steppedStrOut = input.value;
          assert_true(steppedStrOut.startsWith(steppedString),
             "The changed input value [" + steppedStrOut + "] must resemble ["+steppedString+"]");

          // step back down, get first date out again
          input.stepDown()
          var backDown = input.valueAsDate;
          assert_equals(backDown.getTime(), baseMillis, "Stepping back down must return the date to its original value");

       }, inputType + " should step correctly");
    }

    var millis_per_day = 24 * 60 * 60 * 1000;

    // jan 1 midnight, step 1 day to jan 2
    test_stepping("date", "1970-01-01", "1970-01-02", 0, millis_per_day);

    // jan 1 midnight, step 1 minute to 00:01:00
    test_stepping("time", "00:00", "00:01", 0, 60 * 1000);

    // jan 1 midnight, step 31 days to feb 1
    test_stepping("month", "1970-01", "1970-02", 0, 31 * millis_per_day);

    // monday jan 5 1970 midnight, step 7 days to jan 12
    // (this has to start on a monday for stepping up and down to return)
    test_stepping("week", "1970-W02", "1970-W03", 4 * millis_per_day, 7 * millis_per_day);
  </script>
 </body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.no_p_in_scope",
      "message": "No “p” element in scope but a “p” end tag seen.",
      "severity": "Error",
      "span": {
        "byte_end": 450,
        "byte_start": 446,
        "col": 3,
        "line": 15
      }
    },
    {
      "category": "Html",
      "code": "html.form.action.empty",
      "message": "Bad value “” for attribute “action” on element “form”.",
      "severity": "Warning",
      "span": {
        "byte_end": 596,
        "byte_start": 486,
        "col": 3,
        "line": 21
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
  "source_name": "html/semantics/forms/the-input-element/input-valueasdate-stepping.html"
}
```
