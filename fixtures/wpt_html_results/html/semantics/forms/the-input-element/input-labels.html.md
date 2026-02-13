# html/semantics/forms/the-input-element/input-labels.html

Counts:
- errors: 1
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-input-element/input-labels.html",
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
    <h3>input_labels</h3>
  </p>

  <hr>

  <div id="log"></div>

  <form method="post"
      enctype="application/x-www-form-urlencoded"
      action=""
      id="input_form">
    <p><label>Full name:<label>(name)<input name=fn id='input_text1'> <small>Format: First Last</small></label></label></p>
    <p><label>Age: <input name=age type=number min=0 id='input_text2'></label></p>
    <p><label>Post code: <input name=pc> <small>Format: AB12 3CD</small></label></p>
  </form>
  <script>

    var input1 = document.getElementById("input_text1");
    var input2 = document.getElementById("input_text2");

    if (typeof(input1.labels) == "object") {
      if (input1.labels.length == 2 && input2.labels.length == 1) {
        test(function() {
          assert_true(true, "labels attribute is correct.");
        });
      } else {
        test(function() {
          assert_unreached("labels attribute is not correct.");
        });
      }
    } else {
      test(function() {
        assert_unreached("labels attribute is not exist.");
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
        "byte_end": 219,
        "byte_start": 215,
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
        "byte_end": 363,
        "byte_start": 255,
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
  "source_name": "html/semantics/forms/the-input-element/input-labels.html"
}
```
