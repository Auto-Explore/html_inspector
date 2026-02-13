# html/semantics/forms/the-input-element/input-type-checkbox.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-input-element/input-type-checkbox.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<head>
<title>input type checkbox</title>
<link rel="author" title="Gary Gao" href="mailto:angrytoast@gmail.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#checkbox-state-(type=checkbox)">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
</head>
<body>
<div style="display:none;">
  <input id="checkbox_default" type="checkbox" width="20" />

  <input id="checkbox_checked" type="checkbox" checked />

  <input id="checkbox_indeterminate" type="checkbox" />

  <input id="checkbox_default_value" type="checkbox" />
</div>

<div id="log"></div>

<script>
  var checkbox_default = document.getElementById('checkbox_default'),
      checkbox_checked = document.getElementById('checkbox_checked'),
      checkbox_indeterminate = document.getElementById('checkbox_indeterminate'),
      checkbox_default_value = document.getElementById('checkbox_default_value');

  test(function() {
    assert_false(checkbox_default.checked);
  }, "default checkbox has no checkedness state");

  test(function() {
    assert_true(checkbox_checked.checked);
  }, "checkbox with initial state set to checked has checkedness state");

  test(function() {
    checkbox_default.checked = 'chicken'
    assert_true(checkbox_default.checked);
  }, "changing the checked attribute to a string sets the checkedness state");

  test(function() {
    assert_false(checkbox_indeterminate.indeterminate);
  }, "a checkbox has an indeterminate state set to false onload");

  test(function() {
    checkbox_indeterminate.indeterminate = true,
    assert_true(checkbox_indeterminate.indeterminate);
  }, "on setting, a checkbox's indeterminate state must be set to the new value and returns the last value it was set to");

  test(function() {
    assert_equals(checkbox_default_value.value, 'on');
  }, "default/on: on getting, if the element has a value attribute, it must return that attribute's value; otherwise, it must return the string 'on'");

  test(function() {
    checkbox_default_value.value = 'chicken'
    assert_equals(checkbox_default_value.value, 'chicken');
  }, "on getting, if the element has a value attribute, it must return that attribute's value");
</script>

</body>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.input.width.disallowed_for_type",
      "message": "Attribute “width” not allowed on element “input” at this point.",
      "severity": "Warning",
      "span": {
        "byte_end": 435,
        "byte_start": 377,
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
  "source_name": "html/semantics/forms/the-input-element/input-type-checkbox.html"
}
```
