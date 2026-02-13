# html/semantics/forms/the-input-element/input-valueasdate-typeerror.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-input-element/input-valueasdate-typeerror.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<html>
 <head>
  <title>Forms</title>
  <script src="/resources/testharness.js"></script>
  <script src="/resources/testharnessreport.js"></script>
 </head>
 <body>
  <h3>input_valueAsDate_TypeError</h3>
  <hr>
  <div id="log"></div>
  <input id="input_date" type="date">
  <input id="input_time" type="time">
  <input id="input_week" type="week">
  <input id="input_month" type="month">

  <script>
    "use strict";

    function testExpectTypeError(input) {
      test(
        () => assert_throws_js(TypeError, () => input.valueAsDate = {}),
        `valueAsDate setter with non-Date object (input type ${input.type})`,
        'expected TypeError'
      );
    }

    testExpectTypeError(document.getElementById("input_date"));
    testExpectTypeError(document.getElementById("input_time"));
    testExpectTypeError(document.getElementById("input_week"));
    testExpectTypeError(document.getElementById("input_month"));
  </script>

 </body>
</html>
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
  "source_name": "html/semantics/forms/the-input-element/input-valueasdate-typeerror.html"
}
```
