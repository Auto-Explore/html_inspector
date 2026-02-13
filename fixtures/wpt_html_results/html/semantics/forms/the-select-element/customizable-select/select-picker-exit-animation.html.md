# html/semantics/forms/the-select-element/customizable-select/select-picker-exit-animation.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/customizable-select/select-picker-exit-animation.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=help href="https://issues.chromium.org/issues/394133544">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>

<select>
  <option>option</option>
</select>

<style>
select, ::picker(select) {
  appearance: base-select;
}


select.animate::picker(select) {
  transition-property: display, color/*, overlay*/;
  transition-behavior: allow-discrete;
  transition-duration: 100s;
  transition-timing-function: steps(2, start);
  color: white;
}

select.animate option {
  transition-property: background-color;
  transition-duration: 100s;
  transition-timing-function: steps(2, start);
  background-color: black;
}

::picker(select) {
  color: white;
}
option {
  background-color: black;
}

select.animate:not(:open)::picker(select) {
  color: black;
}
select.animate:not(:open) option {
  background-color: white;
}
</style>

<script>
const select = document.querySelector('select');
const option = document.querySelector('option');
promise_test(async () => {
  await test_driver.click(select);
  await new Promise(requestAnimationFrame);
  select.classList.add('animate');
  await new Promise(requestAnimationFrame);
  await test_driver.click(select);
  await new Promise(requestAnimationFrame);
  await new Promise(requestAnimationFrame);

  const style = getComputedStyle(option);
  assert_equals(style.color, 'rgb(128, 128, 128)',
    'color should transition based on the exit animation.');
  assert_equals(style.backgroundColor, 'rgb(128, 128, 128)',
    'background-color should transition based on the exit animation.');
}, 'Top layer exit animations should work on ::picker(select) just like a popover.');
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.style.not_allowed_here",
      "message": "Element “style” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 401,
        "byte_start": 394,
        "col": 1,
        "line": 13
      }
    },
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
  "source_name": "html/semantics/forms/the-select-element/customizable-select/select-picker-exit-animation.html"
}
```
