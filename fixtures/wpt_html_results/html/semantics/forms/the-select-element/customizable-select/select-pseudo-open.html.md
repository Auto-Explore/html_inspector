# html/semantics/forms/the-select-element/customizable-select/select-pseudo-open.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/customizable-select/select-pseudo-open.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=help href="https://github.com/openui/open-ui/issues/547">
<link rel=help href="https://drafts.csswg.org/selectors/#open-state">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="/resources/testdriver-vendor.js"></script>

<select id=myselect>
  <option>one</option>
  <option>two</option>
</select>

<style>
  select, ::picker(select) {
    appearance: base-select;
  }
</style>

<script>
promise_test(async () => {
  assert_true(CSS.supports('appearance', 'base-select'),
    'This test requires appearance:base-select in order to run.');

  assert_false(myselect.matches(':open'),
    'select should not match :open while it is closed.');
  await test_driver.click(myselect);

  assert_true(myselect.matches(':open'),
    'select should match :open while it is open.');
}, 'select should support :open pseudo selector.');
</script>

<select id=selectinvalidation>
  <option>one</option>
  <option>two</option>
</select>
<style>
select:not(:open) {
  background-color: red;
}
select:open {
  background-color: green;
}
</style>

<script>
promise_test(async () => {
  assert_true(CSS.supports('appearance', 'base-select'),
    'This test requires appearance:base-select in order to run.');

  const select = document.getElementById('selectinvalidation');
  const option = select.querySelector('option');

  assert_equals(getComputedStyle(select).backgroundColor, 'rgb(255, 0, 0)',
    'The style rules from :not(:open) should apply when the select is closed.');

  await test_driver.click(select);
  assert_equals(getComputedStyle(select).backgroundColor, 'rgb(0, 128, 0)',
    'The style rules from :open should apply when the select is open.');

  await test_driver.click(select);
  assert_equals(getComputedStyle(select).backgroundColor, 'rgb(255, 0, 0)',
    'The style rules from :not(:open) should apply when the select is opened and closed again.');
}, 'select :open and :not(:open) should invalidate correctly.');
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
        "byte_end": 560,
        "byte_start": 553,
        "col": 1,
        "line": 16
      }
    },
    {
      "category": "Html",
      "code": "html.style.not_allowed_here",
      "message": "Element “style” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1182,
        "byte_start": 1175,
        "col": 1,
        "line": 40
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
  "source_name": "html/semantics/forms/the-select-element/customizable-select/select-pseudo-open.html"
}
```
