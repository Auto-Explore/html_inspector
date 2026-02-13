# html/select/options-length-too-large.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/select/options-length-too-large.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
  <head>
    <meta charset="utf-8">
    <meta name="timeout" content="long">
    <title>select options.length too large</title>

    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
  </head>
  <body>
    <select id="test">
      <option value="1"></option>
      <option value="2"></option>
      <option value="3"></option>
    </select>

    <script>
    var mySelect = document.getElementById("test");

    test(function() {
        mySelect.options.length = -1;
        assert_equals(mySelect.options.length, 3, "Length of <select> should remain unchanged");
    });

    test(function() {
        mySelect.options.length = 100001;
        assert_equals(mySelect.options.length, 3, "Length of <select> should remain unchanged");
    });

    test(function() {
        mySelect.options.length = Number.MAX_SAFE_INTEGER;
        assert_equals(mySelect.options.length, 3, "Length of <select> should remain unchanged");
    });

    test(function() {
        mySelect.options.length = 100000;
        assert_equals(mySelect.options.length, 100000, "Length of <select> should be 100,000");
    });

    test(function() {
        mySelect.appendChild(new Option());
        mySelect.appendChild(new Option());
        assert_equals(mySelect.options.length, 100002, "Manual expansion still works");
        mySelect.options.length = 100001;
        assert_equals(mySelect.options.length, 100001, "Truncation works if over the limit");
    });
    </script>
  </body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.option.empty_without_label",
      "message": "Element “option” without attribute “label” must not be empty.",
      "severity": "Warning",
      "span": {
        "byte_end": 341,
        "byte_start": 332,
        "col": 25,
        "line": 13
      }
    },
    {
      "category": "Html",
      "code": "html.option.empty_without_label",
      "message": "Element “option” without attribute “label” must not be empty.",
      "severity": "Warning",
      "span": {
        "byte_end": 375,
        "byte_start": 366,
        "col": 25,
        "line": 14
      }
    },
    {
      "category": "Html",
      "code": "html.option.empty_without_label",
      "message": "Element “option” without attribute “label” must not be empty.",
      "severity": "Warning",
      "span": {
        "byte_end": 409,
        "byte_start": 400,
        "col": 25,
        "line": 15
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
  "source_name": "html/select/options-length-too-large.html"
}
```
