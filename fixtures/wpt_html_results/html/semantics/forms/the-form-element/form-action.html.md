# html/semantics/forms/the-form-element/form-action.html

Counts:
- errors: 1
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-form-element/form-action.html",
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
    <h3>Form_action</h3>
  </p>

  <hr>

  <div id="log"></div>

  <form method="post"
      enctype="application/x-www-form-urlencoded"
      action="http://www.google.com/"
      id="input_form">
  <p><input type=hidden name="custname"></p>
  <p><input type=hidden name="custtel"></p>
  <p><input type=hidden name="custemail"></p>

  </form>
  <script>

    var form = document.getElementById("input_form");

    if (typeof(form.action) == "string") {
      test(function() {
        assert_equals(form.action, "http://www.google.com/", "action attribute is not correct.");
      });
    } else {
      test(function() {
        assert_unreached("action attribute is not exist.");
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
        "byte_end": 218,
        "byte_start": 214,
        "col": 3,
        "line": 11
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
  "source_name": "html/semantics/forms/the-form-element/form-action.html"
}
```
