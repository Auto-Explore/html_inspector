# html/semantics/forms/the-select-element/selected-index.html

Counts:
- errors: 0
- warnings: 12
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/selected-index.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<title>HTMLSelectElement selectedIndex</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id=log></div>

<form id=form>
  <select id=empty></select>

  <select id=default>
    <option></option>
    <option></option>
    <option></option>
    <option></option>
    <option></option>
  </select>

  <select id=disabled>
    <option disabled></option>
    <option></option>
  </select>

  <select id=selected>
    <option></option>
    <option selected></option>
  </select>

  <select id=display-none>
    <option style="display:none"></option>
    <option></option>
  </select>

  <select id=minus-one>
    <option value=1>1</option>
    <option value=2>2</option>
  </select>
</form>

<script>
function assertSelectedIndex(select, value) {
  assert_equals(select.selectedIndex, value);
  assert_equals(select.options.selectedIndex, value);
}

function assertSelectValue(select, value) {
  assert_equals(select.value, value);
}

test(function () {
  var select = document.getElementById('empty');
  assertSelectedIndex(select, -1);
}, "get empty");

test(function () {
  var select = document.getElementById('default');
  assertSelectedIndex(select, 0);
}, "get default");

test(function () {
  var select = document.getElementById('disabled');
  assertSelectedIndex(select, 1);
}, "get disabled");

test(function () {
  var select = document.getElementById('selected');
  assertSelectedIndex(select, 1);
}, "get unselected");

test(function () {
  var select = document.getElementById('empty');
  select.selectedIndex = 1;
  assertSelectedIndex(select, -1);
}, "set empty (HTMLSelectElement)");

test(function () {
  var select = document.getElementById('empty');
  select.options.selectedIndex = 1;
  assertSelectedIndex(select, -1);
}, "set empty (HTMLOptionsCollection)");

test(function () {
  var select = document.getElementById('default');
  assertSelectedIndex(select, 0);
  select.selectedIndex = 2;
  assertSelectedIndex(select, 2);
  this.add_cleanup(() => { select.selectedIndex = 0; });
}, "set (HTMLSelectElement)");

test(function () {
  var select = document.getElementById('default');
  assertSelectedIndex(select, 0);
  select.options.selectedIndex = 2;
  assertSelectedIndex(select, 2);
  this.add_cleanup(() => { select.selectedIndex = 0; });
}, "set (HTMLOptionsCollection)");

test(function () {
  var select = document.getElementById('selected');
  var form = document.getElementById('form');
  assertSelectedIndex(select, 1);
  select.selectedIndex = 0;
  assertSelectedIndex(select, 0);
  form.reset();
  assertSelectedIndex(select, 1);
}, "set and reset (HTMLSelectElement)");

test(function () {
  var select = document.getElementById('selected');
  var form = document.getElementById('form');
  assertSelectedIndex(select, 1);
  select.options.selectedIndex = 0;
  assertSelectedIndex(select, 0);
  form.reset();
  assertSelectedIndex(select, 1);
}, "set and reset (HTMLOptionsCollection)");

test(function () {
  var select = document.getElementById('display-none');
  assertSelectedIndex(select, 0);
}, "get display:none");

test(function () {
  var select = document.getElementById('display-none');
  select.offsetTop; // force rendering
  assertSelectedIndex(select, 0);
  select.options[1].selected = true;
  assertSelectedIndex(select, 1);
  select.options[1].selected = false;
  assertSelectedIndex(select, 0);
}, "reset to display:none");

test(function() {
  var select = document.getElementById("minus-one");
  assertSelectedIndex(select, 0);

  select.selectedIndex = -1;

  assertSelectedIndex(select, -1);
  assertSelectValue(select, "");

}, "set selectedIndex=-1");
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
        "byte_end": 298,
        "byte_start": 289,
        "col": 13,
        "line": 12
      }
    },
    {
      "category": "Html",
      "code": "html.option.empty_without_label",
      "message": "Element “option” without attribute “label” must not be empty.",
      "severity": "Warning",
      "span": {
        "byte_end": 320,
        "byte_start": 311,
        "col": 13,
        "line": 13
      }
    },
    {
      "category": "Html",
      "code": "html.option.empty_without_label",
      "message": "Element “option” without attribute “label” must not be empty.",
      "severity": "Warning",
      "span": {
        "byte_end": 342,
        "byte_start": 333,
        "col": 13,
        "line": 14
      }
    },
    {
      "category": "Html",
      "code": "html.option.empty_without_label",
      "message": "Element “option” without attribute “label” must not be empty.",
      "severity": "Warning",
      "span": {
        "byte_end": 364,
        "byte_start": 355,
        "col": 13,
        "line": 15
      }
    },
    {
      "category": "Html",
      "code": "html.option.empty_without_label",
      "message": "Element “option” without attribute “label” must not be empty.",
      "severity": "Warning",
      "span": {
        "byte_end": 386,
        "byte_start": 377,
        "col": 13,
        "line": 16
      }
    },
    {
      "category": "Html",
      "code": "html.option.empty_without_label",
      "message": "Element “option” without attribute “label” must not be empty.",
      "severity": "Warning",
      "span": {
        "byte_end": 453,
        "byte_start": 444,
        "col": 22,
        "line": 20
      }
    },
    {
      "category": "Html",
      "code": "html.option.empty_without_label",
      "message": "Element “option” without attribute “label” must not be empty.",
      "severity": "Warning",
      "span": {
        "byte_end": 475,
        "byte_start": 466,
        "col": 13,
        "line": 21
      }
    },
    {
      "category": "Html",
      "code": "html.option.empty_without_label",
      "message": "Element “option” without attribute “label” must not be empty.",
      "severity": "Warning",
      "span": {
        "byte_end": 533,
        "byte_start": 524,
        "col": 13,
        "line": 25
      }
    },
    {
      "category": "Html",
      "code": "html.option.empty_without_label",
      "message": "Element “option” without attribute “label” must not be empty.",
      "severity": "Warning",
      "span": {
        "byte_end": 564,
        "byte_start": 555,
        "col": 22,
        "line": 26
      }
    },
    {
      "category": "Html",
      "code": "html.option.empty_without_label",
      "message": "Element “option” without attribute “label” must not be empty.",
      "severity": "Warning",
      "span": {
        "byte_end": 647,
        "byte_start": 638,
        "col": 34,
        "line": 30
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
        "col": 13,
        "line": 31
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
  "source_name": "html/semantics/forms/the-select-element/selected-index.html"
}
```
