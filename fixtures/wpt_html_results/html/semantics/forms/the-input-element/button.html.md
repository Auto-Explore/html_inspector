# html/semantics/forms/the-input-element/button.html

Counts:
- errors: 0
- warnings: 5
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-input-element/button.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<title>input type button</title>
<link rel="author" title="Denis Ah-Kang" href="mailto:denis@w3.org">
<link rel=help href="https://html.spec.whatwg.org/multipage/#button-state-(type=button)">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<form id=f1>
  <input type=button id=b1 name=b1>
</form>
<form>
  <input id=i1 value="foo">
  <input type=button id=b2 name=b2>
</form>
<form>
  <input type=radio id=i2 checked name=b3>
  <input type=button id=b3 name=b3>
</form>
<form>
  <input type=checkbox id=i3>
  <input type=button id=b4 name=b4>
</form>

<script>
  var t = async_test("clicking on button should not submit a form"),
      b1 = document.getElementById('b1'),
      b2 = document.getElementById('b2'),
      b3 = document.getElementById('b3'),
      b4 = document.getElementById('b4'),
      i1 = document.getElementById('i1'),
      i2 = document.getElementById('i2'),
      i3 = document.getElementById('i3');

  test(function(){
    assert_false(b1.willValidate);
  }, "the element is barred from constraint validation");

  document.getElementById('f1').onsubmit = t.step_func(function(e) {
    e.preventDefault();
    assert_unreached("form has been submitted");
  });

  t.step(function() {
    b1.click();
  });
  t.done();

  test(function(){
    i1.value = "bar";
    b2.click();
    assert_equals(i1.value, "bar");
  }, "clicking on button should not reset other form fields");

  test(function(){
    assert_true(i2.checked);
    b3.click();
    assert_true(i2.checked);
  }, "clicking on button should not unchecked radio buttons");

  test(function(){
    assert_false(i3.indeterminate);
    b4.click();
    assert_false(i3.indeterminate);
  }, "clicking on button should not change its indeterminate IDL attribute");
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.input.button.value.nonempty",
      "message": "Element “input” with attribute “type” whose value is “button” must have non-empty attribute “value”.",
      "severity": "Warning",
      "span": {
        "byte_end": 404,
        "byte_start": 371,
        "col": 3,
        "line": 10
      }
    },
    {
      "category": "Html",
      "code": "html.input.button.value.nonempty",
      "message": "Element “input” with attribute “type” whose value is “button” must have non-empty attribute “value”.",
      "severity": "Warning",
      "span": {
        "byte_end": 483,
        "byte_start": 450,
        "col": 3,
        "line": 14
      }
    },
    {
      "category": "Html",
      "code": "html.input.button.value.nonempty",
      "message": "Element “input” with attribute “type” whose value is “button” must have non-empty attribute “value”.",
      "severity": "Warning",
      "span": {
        "byte_end": 577,
        "byte_start": 544,
        "col": 3,
        "line": 18
      }
    },
    {
      "category": "Html",
      "code": "html.input.button.value.nonempty",
      "message": "Element “input” with attribute “type” whose value is “button” must have non-empty attribute “value”.",
      "severity": "Warning",
      "span": {
        "byte_end": 658,
        "byte_start": 625,
        "col": 3,
        "line": 22
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
  "source_name": "html/semantics/forms/the-input-element/button.html"
}
```
