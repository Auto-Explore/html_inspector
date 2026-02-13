# html/semantics/forms/the-input-element/checkable-active-onblur-with-click.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-input-element/checkable-active-onblur-with-click.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
<meta charset="utf-8">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<style>
* {
  font-size: 20px;
}
</style>
</head>
<body>

<!-- This behavior is not explicitly specified. -->

<input type=checkbox id=cb1 checked> <label for=cb1>ghi</label>
<input type=radio id=r1 checked> <label for=r1>jkl</label>
<label id=lc>abc <input type=checkbox id=cb2 checked></label>
<label id=lr>def <input type=radio id=r2 checked></label>

<script>
promise_test(async () => {
  await new Promise(resolve => {
    addEventListener("load", resolve, { once: true });
  });
}, "Wait for load");

const tabKey = "\uE004";
promise_test(async t => {
  const checkbox = document.querySelector("input[type=checkbox]");
  // pointerdown on the checkbox
  await (new test_driver.Actions()
    .pointerMove(2, 2, { origin: checkbox })
    .pointerDown())
    .send();
  t.add_cleanup(async () => {
    // Release the pointer
    await (new test_driver.Actions().pointerUp()).send();
  });
  assert_equals(document.querySelector("input:active"), checkbox,
    "Checkboxes should be :active while it is pressed");

  // Press tab
  await (new test_driver.Actions().keyDown(tabKey).keyUp(tabKey)).send();
  assert_equals(document.querySelector(":active"), null,
    "Checkboxes should not be :active after tab is used to change focus.");
}, "Checkboxes should clear :active when the user tabs away from them while pressing it with a pointing device");

promise_test(async t => {
  const radio = document.querySelector("input[type=radio]");
  // pointerdown on the radio
  await (new test_driver.Actions()
    .pointerMove(2, 2, { origin: radio })
    .pointerDown())
    .send();
  t.add_cleanup(async () => {
    // Release the pointer
    await (new test_driver.Actions().pointerUp()).send();
  });
  assert_equals(document.querySelector("input:active"), radio,
    "Radios should be :active while it is pressed");

  // Press tab
  await (new test_driver.Actions().keyDown(tabKey).keyUp(tabKey)).send();
  assert_equals(document.querySelector(":active"), null,
    "Radios should not be :active after tab is used to change focus.");
}, "Radios should clear :active when the user tabs away from them while pressing it with a pointing device");

promise_test(async t => {
  const checkbox = document.querySelector("label > input[type=checkbox]");
  const label = checkbox.parentElement;
  // pointerdown on the label
  await (new test_driver.Actions()
    .pointerMove(2, 2, { origin: label })
    .pointerDown())
    .send();
  t.add_cleanup(async () => {
    // Release the pointer
    await (new test_driver.Actions().pointerUp()).send();
  });
  assert_equals(document.querySelector("input:active"), checkbox,
    "Checkboxes should be :active while the label is pressed");

  // Press tab
  await (new test_driver.Actions().keyDown(tabKey).keyUp(tabKey)).send();
  assert_equals(document.querySelector(":active"), null,
    "Checkboxes should not be :active after tab is used to change focus.");
}, "Checkboxes should clear :active when the user tabs away from them while pressing the parent label with a pointing device");

promise_test(async t => {
  const radio = document.querySelector("label > input[type=radio]");
  const label = radio.parentElement;
  const radioRect = radio.getBoundingClientRect();
  const labelRect = label.getBoundingClientRect();
  // pointerdown on the label
  await (new test_driver.Actions()
    .pointerMove(2, 2, { origin: label })
    .pointerDown())
    .send();
  t.add_cleanup(async () => {
    // Release the pointer
    await (new test_driver.Actions().pointerUp()).send();
  });
  assert_equals(document.querySelector("input:active"), radio,
    "Radios should be :active while the label is pressed");

  // Press tab
  await (new test_driver.Actions().keyDown(tabKey).keyUp(tabKey)).send();
  assert_equals(document.querySelector(":active"), null,
    "Radios should not be :active after tab is used to change focus.");
}, "Radios should clear :active when the user tabs away from them while pressing the parent label with a pointing device");

promise_test(async t => {
  const label = document.querySelector("label[for=cb1]");
  // pointerdown on the label
  await (new test_driver.Actions()
    .pointerMove(2, 2, { origin: label })
    .pointerDown())
    .send();
  t.add_cleanup(async () => {
    // Release the pointer
    await (new test_driver.Actions().pointerUp()).send();
  });
  assert_equals(document.querySelector("input:active"), label.control,
    "Checkboxes should be :active while the label is pressed");

  // Press tab
  await (new test_driver.Actions().keyDown(tabKey).keyUp(tabKey)).send();
  assert_equals(document.querySelector(":active"), null,
    "Checkboxes should not be :active after tab is used to change focus.");
}, "Checkboxes should clear :active when the user tabs away from them while pressing the following label with a pointing device");

promise_test(async t => {
  const label = document.querySelector("label[for=r1]");
  // pointerdown on the label
  await (new test_driver.Actions()
    .pointerMove(2, 2, { origin: label })
    .pointerDown())
    .send();
  t.add_cleanup(async () => {
    // Release the pointer
    await (new test_driver.Actions().pointerUp()).send();
  });
  assert_equals(document.querySelector("input:active"), label.control,
    "Radios should be :active while the label is pressed");

  // Press tab
  await (new test_driver.Actions().keyDown(tabKey).keyUp(tabKey)).send();
  assert_equals(document.querySelector(":active"), null,
    "Radios should not be :active after tab is used to change focus.");
}, "Radios should clear :active when the user tabs away from them while pressing the following label with a pointing device");
</script>
</body>
</html>
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
  "source_name": "html/semantics/forms/the-input-element/checkable-active-onblur-with-click.html"
}
```
