# html/semantics/forms/the-select-element/select-required-attribute.tentative.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/select-required-attribute.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html lang="en">
<title>HTMLselectElement Test: required attribute</title>
<link rel="author" title="Ionel Popescu" href="mailto:iopopesc@microsoft.com">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<style>
  select:required {
    border: 3px dashed rgb(255, 0, 0);
  }

  select:optional {
    border: 1px solid rgb(128, 128, 128);
  }

  select, ::picker(select) {
    appearance: base-select;
  }
</style>

<select id="select0" required>
  <option>one</option>
  <option>two</option>
  <option>three</option>
</select>

<select id="select1">
  <option>one</option>
  <option>two</option>
  <option>three</option>
</select>

<select id="select2">
  <option>one</option>
  <option>two</option>
  <option>three</option>
</select>

<script>
function checkRequired(style) {
  assert_equals(style.borderWidth, '3px');
  assert_equals(style.borderStyle, 'dashed');
  assert_equals(style.borderColor, 'rgb(255, 0, 0)');
}

function checkOptional(style) {
  assert_equals(style.borderWidth, '1px');
  assert_equals(style.borderStyle, 'solid');
  assert_equals(style.borderColor, 'rgb(128, 128, 128)');
}

test(() => {
  const select0 = document.getElementById("select0");
  const select1 = document.getElementById("select1");
  const select2 = document.getElementById("select2");

  checkRequired(window.getComputedStyle(select0));
  checkOptional(window.getComputedStyle(select1));
  checkOptional(window.getComputedStyle(select2));
  select2.required = true;
  checkRequired(window.getComputedStyle(select2));
}, "Test required attribute");

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
        "byte_end": 599,
        "byte_start": 590,
        "col": 1,
        "line": 26
      }
    }
  ],
  "source_name": "html/semantics/forms/the-select-element/select-required-attribute.tentative.html"
}
```
