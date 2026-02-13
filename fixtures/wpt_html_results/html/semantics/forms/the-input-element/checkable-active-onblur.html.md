# html/semantics/forms/the-input-element/checkable-active-onblur.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-input-element/checkable-active-onblur.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<link rel="author" title="Joey Arhar" href="mailto:jarhar@chromium.org">
<link rel="help" href="http://crbug.com/1157510">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="/resources/testdriver-vendor.js"></script>

<!-- This behavior is not explicitly specified. -->

<input type=checkbox id=checkbox checked>
<input type=radio id=radio checked>

<script>
promise_test(async t => {
  checkbox.focus();

  // Hold spacebar down
  await (new test_driver.Actions()).keyDown('\uE00D').send();
  t.add_cleanup(async () => {
    // Release spacebar
    await (new test_driver.Actions()).keyUp('\uE00D').send();
  });
  assert_equals(document.querySelector('input:active'), checkbox,
    'Checkboxes should be :active while the spacebar is pressed down.');

  // Press tab
  await (new test_driver.Actions()).keyDown('\uE004').keyUp('\uE004').send();
  assert_equals(document.querySelector(':active'), null,
    'Checkboxes should not be :active after tab is used to change focus.');
}, 'Checkboxes should clear :active when the user tabs away from them while holding spacebar.');

promise_test(async t => {
  radio.focus();

  // Hold spacebar down
  await (new test_driver.Actions()).keyDown('\uE00D').send();
  t.add_cleanup(async () => {
    // Release spacebar
    await (new test_driver.Actions()).keyUp('\uE00D').send();
  });
  assert_equals(document.querySelector('input:active'), radio,
    'Radios should be :active while the spacebar is pressed down.');

  // Press tab
  await (new test_driver.Actions()).keyDown('\uE004').keyUp('\uE004').send();
  assert_equals(document.querySelector(':active'), null,
    'Radios should not be :active after tab is used to change focus.');
}, 'Radios should clear :active when the user tabs away from them while holding spacebar.');
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
  "source_name": "html/semantics/forms/the-input-element/checkable-active-onblur.html"
}
```
