# html/semantics/forms/the-input-element/input-list.html

Counts:
- errors: 3
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-input-element/input-list.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<html>
 <head>
  <title>input list attribute</title>
  <script src="/resources/testharness.js"></script>
  <script src="/resources/testharnessreport.js"></script>
 </head>
 <body>
  <p>
    <h3>input_list</h3>
  </p>

  <hr>

  <div id="log"></div>

  <form method="post"
      enctype="application/x-www-form-urlencoded"
      action=""
      name="input_form">
    <datalist id="thelist">
      <option value="one">one</option>
      <option value="two">two</option>
    </datalist>

    <p id="non_datalist_first">
    <datalist id="non_datalist_first">
      <option value="one">one</option>
      <option value="two">two</option>
    </datalist>

    <datalist id="datalist_first">
      <option value="one">one</option>
      <option value="two">two</option>
    </datalist>
    <p id="datalist_first">

    <p><input list="thelist" id='input_with_list'></p>
    <p><input id='input_without_list'></p>
    <p><input list="input_with_list" id='input_with_nondatalist_list'></p>
    <p><input list="not_an_id" id='input_with_missing_list'></p>
    <p><input list="non_datalist_first" id='input_with_non_datalist_first'></p>
    <p><input list="datalist_first" id='input_with_datalist_first'></p>
  </form>

  <script>
     test(function() {
        assert_equals(document.getElementById("input_with_list").list, document.getElementById("thelist"));
     }, "getting .list of input must return the datalist with that id");
     test(function() {
        assert_equals(document.getElementById("input_without_list").list, null);
     }, "getting .list of input must return null if it has no list attribute");
     test(function() {
        assert_equals(document.getElementById("input_with_nondatalist_list").list, null);
     }, "getting .list of input must return null if the list attribute is a non-datalist's id");
     test(function() {
        assert_equals(document.getElementById("input_with_missing_list").list, null);
     }, "getting .list of input must return null if the list attribute is no element's id");
     test(function() {
        assert_equals(document.getElementById("input_with_non_datalist_first").list, null);
     }, "getting .list of input must return null if the list attribute is used in a non-datalist earlier than a datalist");
     test(function() {
        assert_equals(document.getElementById("input_with_datalist_first").list, document.querySelector("datalist#datalist_first"));
     }, "getting .list of input must return the datalist with that id even if a later non-datalist also has the id");
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
        "byte_end": 232,
        "byte_start": 228,
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
        "byte_end": 378,
        "byte_start": 268,
        "col": 3,
        "line": 17
      }
    },
    {
      "category": "Html",
      "code": "html.id.duplicate",
      "message": "Duplicate ID “non_datalist_first”.",
      "severity": "Error",
      "span": {
        "byte_end": 572,
        "byte_start": 538,
        "col": 5,
        "line": 27
      }
    },
    {
      "category": "Html",
      "code": "html.id.duplicate",
      "message": "Duplicate ID “datalist_first”.",
      "severity": "Error",
      "span": {
        "byte_end": 833,
        "byte_start": 830,
        "col": 5,
        "line": 38
      }
    },
    {
      "category": "Html",
      "code": "html.input.list.must_refer_to_datalist",
      "message": "The “list” attribute of the “input” element must refer to a “datalist” element.",
      "severity": "Warning",
      "span": {
        "byte_end": 994,
        "byte_start": 931,
        "col": 8,
        "line": 40
      }
    },
    {
      "category": "Html",
      "code": "html.input.list.must_refer_to_datalist",
      "message": "The “list” attribute of the “input” element must refer to a “datalist” element.",
      "severity": "Warning",
      "span": {
        "byte_end": 1059,
        "byte_start": 1006,
        "col": 8,
        "line": 41
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
  "source_name": "html/semantics/forms/the-input-element/input-list.html"
}
```
