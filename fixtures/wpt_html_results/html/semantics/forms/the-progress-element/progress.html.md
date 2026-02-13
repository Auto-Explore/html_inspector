# html/semantics/forms/the-progress-element/progress.html

Counts:
- errors: 0
- warnings: 6
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-progress-element/progress.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
  <head>
    <title>The progress element</title>

    <link rel="author" title="dan smith" href="mailto:XX1011@gmail.com">
    <link rel="author" title="Tomoyuki SHIMIZU" href="mailto:tomoyuki.labs@gmail.com">
    <link rel="help" href="https://html.spec.whatwg.org/multipage/#the-progress-element">

    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
  </head>
  <body>
    <div id="log"></div>

    <progress id="indeterminate"></progress>
    <progress id="removevalue" value="0.5"></progress>
    <progress id="quarter" value="1" max="4"></progress>
    <progress id="largerthanmax" value="2"></progress>
    <progress id="invalidmax" value="1" max="a"></progress>
    <progress id="negativemax" value="1" max="-1"></progress>
    <progress id="invalidvalue" value="a"></progress>
    <progress id="negativevalue" value="-1"></progress>

    <script>

      test(function() {
        assert_equals(indeterminate.position, -1);
      }, "Indeterminate progress bar should have position -1");

      test(function() {
        removevalue.removeAttribute('value');
        assert_equals(removevalue.position, -1);
      }, "Revoming the value attribute makes an intermediate progress bar, which should have position -1");

      test(function() {
        assert_equals(quarter.position, quarter.value / quarter.max);
      }, "Determinate progress bar should have fractional position");

      test(function() {
        assert_equals(indeterminate.value, 0);
      }, "Indeterminate progress bar should have value 0");

      test(function() {
        assert_equals(largerthanmax.value, 1);
      }, "Value must equal max if the parsed value is larger than max");

      test(function() {
        assert_equals(indeterminate.max, 1);
      }, "Max must be 1 by default");

      test(function() {
        assert_equals(largerthanmax.max, 1);
      }, "Max must be 1 by default, even if value is specified");

      test(function() {
        assert_equals(invalidmax.max, 1);
      }, "Max must be 1 if max value is invalid");

      test(function() {
        assert_equals(negativemax.max, 1);
      }, "Max must be 1 if the parsed max value is less than or equal to zero");

      test(function() {
        assert_equals(invalidvalue.value, 0);
      }, "Value must be 0 if value is invalid");

      test(function() {
        assert_equals(negativevalue.value, 0);
      }, "Value must be 0 if the parsed value is less than or equal to zero");

    </script>
  </body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.progress.value.exceeds_one",
      "message": "The value of the  “value” attribute must be less than or equal to one when the “max” attribute is absent.",
      "severity": "Warning",
      "span": {
        "byte_end": 683,
        "byte_start": 644,
        "col": 5,
        "line": 19
      }
    },
    {
      "category": "Html",
      "code": "html.progress.max.positive",
      "message": "Bad value “a” for attribute “max” on element “progress”.",
      "severity": "Warning",
      "span": {
        "byte_end": 743,
        "byte_start": 699,
        "col": 5,
        "line": 20
      }
    },
    {
      "category": "Html",
      "code": "html.progress.max.positive",
      "message": "Bad value “-1” for attribute “max” on element “progress”.",
      "severity": "Warning",
      "span": {
        "byte_end": 805,
        "byte_start": 759,
        "col": 5,
        "line": 21
      }
    },
    {
      "category": "Html",
      "code": "html.progress.value.invalid",
      "message": "Bad value “a” for attribute “value” on element “progress”.",
      "severity": "Warning",
      "span": {
        "byte_end": 859,
        "byte_start": 821,
        "col": 5,
        "line": 22
      }
    },
    {
      "category": "Html",
      "code": "html.progress.value.non_negative",
      "message": "Bad value “-1” for attribute “value” on element “progress”.",
      "severity": "Warning",
      "span": {
        "byte_end": 915,
        "byte_start": 875,
        "col": 5,
        "line": 23
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
  "source_name": "html/semantics/forms/the-progress-element/progress.html"
}
```
