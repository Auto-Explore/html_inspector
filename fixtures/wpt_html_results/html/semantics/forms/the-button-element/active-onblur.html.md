# html/semantics/forms/the-button-element/active-onblur.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-button-element/active-onblur.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<link rel="author" title="Joey Arhar" href="mailto:jarhar@chromium.org">
<link rel="help" href="http://crbug.com/945854">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="/resources/testdriver-vendor.js"></script>

<!-- This behavior is not explicitly specified. -->

<button id=b1>button one</button>
<button id=b2>button two</button>

<script>
promise_test(async () => {
  b1.focus();

  // Hold spacebar down
  await (new test_driver.Actions()).keyDown('\uE00D').send();
  assert_equals(document.querySelector(':active'), b1,
    'Buttons should be :active while the spacebar is pressed down.');

  // Press tab
  await (new test_driver.Actions()).keyDown('\uE004').keyUp('\uE004').send();
  assert_equals(document.querySelector(':active'), null,
    'Buttons should not be :active after tab is used to change focus.');

  // Release spacebar
  await (new test_driver.Actions()).keyUp('\uE00D').send();
}, 'Buttons should clear :active when the user tabs away from them while holding spacebar.');
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
  "source_name": "html/semantics/forms/the-button-element/active-onblur.html"
}
```
