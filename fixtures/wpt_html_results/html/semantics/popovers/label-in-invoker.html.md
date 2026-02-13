# html/semantics/popovers/label-in-invoker.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/popovers/label-in-invoker.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=help href="https://bugs.chromium.org/p/chromium/issues/detail?id=1523168">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>

<button popovertarget=mypopover>
  <label>label</label>
</button>
<div id=mypopover popover=auto>popover</div>

<script>
promise_test(async() => {
  const label = document.querySelector('label');
  assert_false(mypopover.matches(':popover-open'),
    'Popover should be closed at the start of the test.');
  await test_driver.click(label);
  assert_true(mypopover.matches(':popover-open'),
    'The popover should be opened by clicking on the label.');
}, 'Buttons with popovertarget should invoke targets even if there is a label in the button.');
</script>
```

```json
{
  "messages": [
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
  "source_name": "html/semantics/popovers/label-in-invoker.html"
}
```
