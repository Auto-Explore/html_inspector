# html/semantics/forms/form-control-infrastructure/form_owner_and_table_2.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/form-control-infrastructure/form_owner_and_table_2.html",
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
    <div>
      <form id='form1'></form>
      <table id='table1'>
        <form id='form2'>
        <script>
          var t = document.getElementById('table1');
          var f = document.getElementById('form2');
          t.removeChild(f);
        </script>
        <tr><td><input id='input1'></td></tr>
        <tr><td><input id='input2' form='form1'></td></tr>
      </table>
      <form id="form3">
        <input id="input3" />
      </form>
    </div>

    <script>
      test(function() {
        var form1 = document.getElementById('form1');

        assert_equals(document.getElementById('input1').form, null,
          "input1's form owner must be null since form2 is not in the" +
          "same home subtree");

        assert_equals(document.getElementById('input2').form, form1,
          "input2's form owner must be the form with id 'form1'");

        assert_equals(document.getElementById('input3').form, null,
          "input3's form owner must be null instead of form2 (as per parsing rules)" +
          "since form2 is not in the same home subtree");

      }, "Controls nested in tables are not associated with form element inside the " +
         "table if the form had been removed by script before the controls were " +
         "inserted by the parser");
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
  "source_name": "html/semantics/forms/form-control-infrastructure/form_owner_and_table_2.html"
}
```
