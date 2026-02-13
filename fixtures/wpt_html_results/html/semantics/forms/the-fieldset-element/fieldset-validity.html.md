# html/semantics/forms/the-fieldset-element/fieldset-validity.html

Counts:
- errors: 1
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-fieldset-element/fieldset-validity.html",
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
    <h3>FieldSet_validity</h3>
  </p>

  <hr>

  <div id="log"></div>

  <form method="post"
      enctype="application/x-www-form-urlencoded"
      action=""
      id="input_form">
    <fieldset id="input_field">
     </fieldset>
  </form>
  <script>

    var field = document.getElementById("input_field");

    if (typeof(field.validity) == "object") {
      test(function() {
        assert_equals(field.validity.valueMissing, false, "validity attribute is not correct.");
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
        "byte_end": 224,
        "byte_start": 220,
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
        "byte_end": 368,
        "byte_start": 260,
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
  "source_name": "html/semantics/forms/the-fieldset-element/fieldset-validity.html"
}
```
