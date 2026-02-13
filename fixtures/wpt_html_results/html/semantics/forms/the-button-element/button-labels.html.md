# html/semantics/forms/the-button-element/button-labels.html

Counts:
- errors: 1
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-button-element/button-labels.html",
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
    <h3>button_labels</h3>
  </p>

  <hr>

  <div id="log"></div>

  <form method="post"
      enctype="application/x-www-form-urlencoded"
      action=""
      id="input_form">
    <p><label>Full name:<label>(name)<button id='button_id1'>button1</button><small>Format: First Last</small></label></label></p>
    <p><label>Age: <button id='button_id2'>button2</button></label></p>
  </form>
  <script>
    test(function() {
      var button1 = document.getElementById("button_id1");
      var button2 = document.getElementById("button_id2");

      assert_true(button1.labels instanceof NodeList, "button1.labels is NodeList");
      assert_equals(button1.labels.length, 2, "button1.labels.length");

      assert_true(button2.labels instanceof NodeList, "button2.labels is NodeList");
      assert_equals(button2.labels.length, 1, "button2.labels.length");
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
      "code": "html.parser.no_p_in_scope",
      "message": "No “p” element in scope but a “p” end tag seen.",
      "severity": "Error",
      "span": {
        "byte_end": 220,
        "byte_start": 216,
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
        "byte_end": 364,
        "byte_start": 256,
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
  "source_name": "html/semantics/forms/the-button-element/button-labels.html"
}
```
