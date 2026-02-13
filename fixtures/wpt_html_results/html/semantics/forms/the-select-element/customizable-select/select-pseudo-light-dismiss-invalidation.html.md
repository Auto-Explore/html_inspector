# html/semantics/forms/the-select-element/customizable-select/select-pseudo-light-dismiss-invalidation.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/customizable-select/select-pseudo-light-dismiss-invalidation.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=help href="http://crbug.com/1429839">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="/resources/testdriver-vendor.js"></script>

<select id=select>
  <option id=optone>one</option>
  <option id=opttwo>two</option>
</select>
<style>
select {
  background-color: rgb(0, 0, 255);
}
select:not(:open) {
  background-color: rgb(0, 255, 0);
}
select:open {
  background-color: rgb(255, 0, 0);
}
select, ::picker(select) {
  appearance: base-select;
}
</style>
<button id=button>option</button>

<script>
function click(element) {
  return (new test_driver.Actions()
    .pointerMove(0, 0, {origin: element})
    .pointerDown()
    .pointerUp())
    .send();
}

promise_test(async () => {
  assert_equals(getComputedStyle(select).backgroundColor, 'rgb(0, 255, 0)',
    'The select should match :not(:open) at the start of the test.');

  await click(select);
  assert_equals(getComputedStyle(select).backgroundColor, 'rgb(255, 0, 0)',
    'The select should match :open when opened.');

  await click(opttwo);
  assert_equals(getComputedStyle(select).backgroundColor, 'rgb(0, 255, 0)',
    'The select should match :not(:open) after clicking an option.');

  await click(select);
  assert_equals(getComputedStyle(select).backgroundColor, 'rgb(255, 0, 0)',
    'The select should match :open when reopened.');

  await click(button);
  assert_equals(getComputedStyle(select).backgroundColor, 'rgb(0, 255, 0)',
    'The select should match :not(:open) after light dismiss.');
}, 'select should not match :open when light dismissed.');
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
        "byte_end": 487,
        "byte_start": 480,
        "col": 1,
        "line": 14
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
  "source_name": "html/semantics/forms/the-select-element/customizable-select/select-pseudo-light-dismiss-invalidation.html"
}
```
