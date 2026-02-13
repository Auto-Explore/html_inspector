# html/semantics/forms/the-select-element/select-value.html

Counts:
- errors: 0
- warnings: 8
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/select-value.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<title>HTMLSelectElement.value</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/forms.html#dom-select-value">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id=log></div>

<select id=sel1>
  <option value=0></option>
  <option selected value=1></option>
</select>

<select id=sel2>
  <optgroup>
    <option value=0></option>
  </optgroup>
  <optgroup></optgroup>
  <optgroup>
    <option></option>
    <option value=1></option>
    <option selected value=2></option>
  </optgroup>
</select>

<select id=sel3>
  <option selected value=1></option>
</select>

<select id=sel4></select>

<script>
test(function() {
  var select = document.getElementById('sel1');
  assert_equals(select.value, '1');
}, 'options');

test(function() {
  var select = document.getElementById('sel2');
  assert_equals(select.value, '2');
}, 'optgroups');

test(function() {
  var select = document.getElementById('sel3');
  var option = select.options[0];
  var div = document.createElement('div');
  select.appendChild(div);
  div.appendChild(option);
  assert_equals(select.value, '1');
}, 'option is child of div');

test(function() {
  var select = document.getElementById('sel4');
  assert_equals(select.value, '');
}, 'no options');
</script>
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
        "byte_end": 340,
        "byte_start": 331,
        "col": 19,
        "line": 10
      }
    },
    {
      "category": "Html",
      "code": "html.option.empty_without_label",
      "message": "Element “option” without attribute “label” must not be empty.",
      "severity": "Warning",
      "span": {
        "byte_end": 377,
        "byte_start": 368,
        "col": 28,
        "line": 11
      }
    },
    {
      "category": "Html",
      "code": "html.option.empty_without_label",
      "message": "Element “option” without attribute “label” must not be empty.",
      "severity": "Warning",
      "span": {
        "byte_end": 448,
        "byte_start": 439,
        "col": 21,
        "line": 16
      }
    },
    {
      "category": "Html",
      "code": "html.option.empty_without_label",
      "message": "Element “option” without attribute “label” must not be empty.",
      "severity": "Warning",
      "span": {
        "byte_end": 521,
        "byte_start": 512,
        "col": 13,
        "line": 20
      }
    },
    {
      "category": "Html",
      "code": "html.option.empty_without_label",
      "message": "Element “option” without attribute “label” must not be empty.",
      "severity": "Warning",
      "span": {
        "byte_end": 551,
        "byte_start": 542,
        "col": 21,
        "line": 21
      }
    },
    {
      "category": "Html",
      "code": "html.option.empty_without_label",
      "message": "Element “option” without attribute “label” must not be empty.",
      "severity": "Warning",
      "span": {
        "byte_end": 590,
        "byte_start": 581,
        "col": 30,
        "line": 22
      }
    },
    {
      "category": "Html",
      "code": "html.option.empty_without_label",
      "message": "Element “option” without attribute “label” must not be empty.",
      "severity": "Warning",
      "span": {
        "byte_end": 669,
        "byte_start": 660,
        "col": 28,
        "line": 27
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
  "source_name": "html/semantics/forms/the-select-element/select-value.html"
}
```
