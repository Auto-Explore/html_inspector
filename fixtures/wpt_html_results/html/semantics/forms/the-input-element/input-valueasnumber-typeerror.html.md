# html/semantics/forms/the-input-element/input-valueasnumber-typeerror.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-input-element/input-valueasnumber-typeerror.html",
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
  <h3>input_valueAsNumber_TypeError</h3>
  <hr>
  <div id="log"></div>

  <input id="input_number" type="number" />
  <input id="input_checkbox" type="checkbox" />

  <script>
    "use strict";

    function testExpectTypeError(input, input_type_applies, values) {
      for (const value of values) {
        test(
          () => assert_throws_js(TypeError, () => input.valueAsNumber = value),
          `valueAsNumber = ${value} (input type ${input_type_applies})`,
          'expected TypeError'
        );
      }
    }

    const input_number = document.getElementById("input_number");
    testExpectTypeError(input_number, "does apply", [Infinity, -Infinity]);

    const input_checkbox = document.getElementById("input_checkbox");
    testExpectTypeError(input_checkbox, "does not apply", [Infinity, -Infinity]);
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
  "source_name": "html/semantics/forms/the-input-element/input-valueasnumber-typeerror.html"
}
```
