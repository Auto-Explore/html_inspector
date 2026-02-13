# html/semantics/forms/the-select-element/select-named-getter.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/select-named-getter.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<title>Absence of a named getter on HTMLSelectElement</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<div id=log></div>
<select id=select>
  <option id=op1>A</option>
  <option name=op2>B</option>
  <option id=op3 name=op4>C</option>
  <option id=>D</option>
  <option name=>D</option>
</select>
<script>
test(function() {
  var select = document.getElementById("select");
  assert_equals(select.op1, undefined);
  assert_false("op1" in select);
  assert_equals(select.namedItem("op1"), select.children[0]);
}, "Option with id")

test(function() {
  var select = document.getElementById("select");
  assert_equals(select.op2, undefined);
  assert_false("op2" in select);
  assert_equals(select.namedItem("op2"), select.children[1]);
}, "Option with name")

test(function() {
  var select = document.getElementById("select");
  assert_equals(select.op3, undefined);
  assert_false("op3" in select);
  assert_equals(select.namedItem("op3"), select.children[2]);

  assert_equals(select.op4, undefined);
  assert_false("op4" in select);
  assert_equals(select.namedItem("op4"), select.children[2]);
}, "Option with name and id")

test(function() {
  var select = document.getElementById("select");
  assert_equals(select[""], undefined);
  assert_false("" in select);
  assert_equals(select.namedItem(""), null);
}, "Empty string name");
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.tokenizer.attr_value_missing",
      "message": "Attribute value missing.",
      "severity": "Warning",
      "span": {
        "byte_end": 348,
        "byte_start": 336,
        "col": 3,
        "line": 11
      }
    },
    {
      "category": "Html",
      "code": "html.id.invalid",
      "message": "Bad value “” for attribute “id” on element “option”.",
      "severity": "Warning",
      "span": {
        "byte_end": 348,
        "byte_start": 336,
        "col": 3,
        "line": 11
      }
    },
    {
      "category": "Html",
      "code": "html.tokenizer.attr_value_missing",
      "message": "Attribute value missing.",
      "severity": "Warning",
      "span": {
        "byte_end": 375,
        "byte_start": 361,
        "col": 3,
        "line": 12
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
  "source_name": "html/semantics/forms/the-select-element/select-named-getter.html"
}
```
