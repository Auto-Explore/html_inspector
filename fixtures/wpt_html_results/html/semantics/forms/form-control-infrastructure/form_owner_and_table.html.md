# html/semantics/forms/form-control-infrastructure/form_owner_and_table.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/form-control-infrastructure/form_owner_and_table.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
  <head>
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
  </head>
  <body>
    <div id="root">
      <form id='form1'></form>
      <table id='table1'>
        <form id='form2'>
        <tr><td><input id='input1'></td></tr>
        <tr><td><input id='input2' form='form1'></td></tr>
      </table>
      <form id="form3">
        <input id="input3" />
      </form>
    </div>

    <script>
      test(function() {
        var input1 = document.getElementById('input1');
        var input2 = document.getElementById('input2');
        var input3 = document.getElementById('input3');
        var form1 = document.getElementById('form1');
        var form2 = document.getElementById('form2');
        var form3 = document.getElementById('form3');

        var root = document.getElementById('root');

        assert_equals(input1.form, form2,
          "input1's form owner must be form2 as per the parsing rules");
        assert_equals(input2.form, form1,
          "input2's form owner must be the form with id 'form1'");
        assert_equals(input3.form, form2,
          "input3's form owner must be form2 as per the parsing rules");

        root.parentNode.removeChild(root);

        assert_equals(input1.form, form2,
          "input1's form owner must not have changed since they are both in the same subtree");
        assert_equals(input2.form, null,
          "input2 must not have a form owner since it has the form attribute set");
        assert_equals(input3.form, form2,
          "input3's form owner must not have changed since they are both in the same subtree");

      }, "Form element and form controls nested inside a table are correctly handled");
    </script>
  </body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.head.title.missing",
      "message": "Element “head” is missing a required instance of child element “title”.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/forms/form-control-infrastructure/form_owner_and_table.html"
}
```
