# html/semantics/forms/the-select-element/select-validity.tentative.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/select-validity.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html lang="en">
<title>HTMLselectElement Test: validity</title>
<link rel="author" title="Ionel Popescu" href="mailto:iopopesc@microsoft.com">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<select id="select1" required>
  <option>one</option>
  <option>two</option>
  <option>three</option>
  <option>four</option>
</select>

<form>
  <select id="select2" required>
  </select>
</form>

<style>
  select, ::picker(select) {
    appearance: base-select;
  }
</style>

<script>

test(() => {
  let select = document.createElement('select');
  assert_true(select.willValidate, "A select element is a submittable element that is a candidate for constraint validation.");
  let option = document.createElement('option');
  select.appendChild(option);
  assert_true(select.checkValidity(), "Always valid when the select isn't a required value.");

  select.required = true;
  assert_equals(select.value, "");
  assert_false(select.checkValidity(), "A selected placeholder option should invalidate the select.");

  let emptyOption = document.createElement('option');
  select.appendChild(emptyOption);
  assert_false(select.checkValidity(), "A selected placeholder option should invalidate the select even if there are multiple options.");
  emptyOption.selected = true;
  assert_true(select.checkValidity(), "An empty non-placeholder option should be a valid choice.");

  let filledOption = document.createElement('option');
  filledOption.value = "test";
  select.appendChild(filledOption);
  filledOption.selected = true;
  assert_equals(select.value, "test", "The non-empty value should be set.");
  assert_true(select.checkValidity(), "A non-empty non-placeholder option should be a valid choice.");

  select.removeChild(option);
  select.appendChild(emptyOption);
  emptyOption.selected = true;
  assert_equals(select.value, "", "The empty value should be set.");
  assert_true(select.checkValidity(), "Only the first option can be seen as a placeholder.");

  select.removeChild(filledOption);
  assert_false(select.checkValidity(), "A selected placeholder option should invalidate the select.");

  emptyOption.value = "test2";
  assert_equals(select.value, "test2");
  assert_true(select.checkValidity(), "A non-empty option value should be a valid choice.");

  emptyOption.removeAttribute("value");
  assert_equals(select.value, "");
  assert_false(select.checkValidity());
  emptyOption.innerText = "test";
  assert_equals(select.value, "test");
  assert_true(select.checkValidity(), "A non-empty option should be a valid choice.");

  const select1 = document.getElementById('select1');
  assert_equals(select1.value, "one");
  assert_true(select1.checkValidity(), "A select with non-empty placeholder option should be valid.");
}, "Validation for placeholder option");

test(() => {
  const select2 = document.getElementById('select2');
  assert_equals(select2.value, "");
  assert_false(select2.checkValidity());
  let form = document.querySelector('form');
  let invalidControl = form.querySelector('select:invalid');
  assert_equals(select2, invalidControl);
  let didDispatchInvalid = false;
  invalidControl.addEventListener('invalid', e => { didDispatchInvalid = true; });
  let didDispatchSubmit = false;
  form.addEventListener('submit', event => { event.preventDefault(); didDispatchSubmit = true; });

  form.requestSubmit();
  assert_true(didDispatchInvalid);
  assert_false(didDispatchSubmit);
}, "Check form not submitted for invalid select");

</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.select.required.first_option.placeholder",
      "message": "The first child “option” element of a “select” element with a “required” attribute, and without a “multiple” attribute, and without a “size” attribute whose value is greater than “1”, must have either an empty “value” attribute, or must have no text content. Consider either adding a placeholder option label, or adding a “size” attribute with a value equal to the number of “option” elements.",
      "severity": "Warning",
      "span": {
        "byte_end": 402,
        "byte_start": 393,
        "col": 1,
        "line": 13
      }
    },
    {
      "category": "Html",
      "code": "html.select.required.must_have_option",
      "message": "A “select” element with a “required” attribute, and without a “multiple” attribute, and without a “size” attribute whose value is greater than “1”, must have a child “option” element.",
      "severity": "Warning",
      "span": {
        "byte_end": 455,
        "byte_start": 446,
        "col": 3,
        "line": 17
      }
    },
    {
      "category": "Html",
      "code": "html.style.not_allowed_here",
      "message": "Element “style” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 472,
        "byte_start": 465,
        "col": 1,
        "line": 20
      }
    }
  ],
  "source_name": "html/semantics/forms/the-select-element/select-validity.tentative.html"
}
```
