# html/semantics/forms/the-input-element/input-validity.html

Counts:
- errors: 1
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-input-element/input-validity.html",
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
  <p>
    <h3>input_validity</h3>
  </p>

  <hr>

  <div id="log"></div>

  <form method="post"
      enctype="application/x-www-form-urlencoded"
      action=""
      id="input_form">
    <p><input type='hidden' id='input_text'></p>
  </form>
  <script>

    var input = document.getElementById("input_text");

    if (typeof(input.validity) == "object") {
      test(function() {
        assert_equals(input.validity.valueMissing, false, "validity attribute is not correct.");
      });
    } else {
      test(function() {
        assert_unreached("validity attribute is not exist.");
      });
    }

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
        "byte_end": 221,
        "byte_start": 217,
        "col": 3,
        "line": 11
      }
    },
    {
      "category": "Html",
      "code": "html.form.action.empty",
      "message": "Bad value “” for attribute “action” on element “form”.",
      "severity": "Warning",
      "span": {
        "byte_end": 365,
        "byte_start": 257,
        "col": 3,
        "line": 17
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
  "source_name": "html/semantics/forms/the-input-element/input-validity.html"
}
```
