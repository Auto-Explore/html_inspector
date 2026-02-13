# html/semantics/forms/the-input-element/checkable-active-space-key-prevented-default.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-input-element/checkable-active-space-key-prevented-default.html",
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
<title>Tests active state of checkbox/radio when pressing space key but its default prevented</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
</head>
<body>
<input type="checkbox">
<input type="radio">
<script>
const spaceKey = "\uE00D";

function preventDefault(event) {
  event.preventDefault();
}

promise_test(async t => {
  const checkbox = document.querySelector("input[type=checkbox]");
  checkbox.focus();
  checkbox.addEventListener("keydown", preventDefault);
  await (new test_driver.Actions()).keyDown(spaceKey).send();
  t.add_cleanup(async () => {
    await (new test_driver.Actions()).keyUp(spaceKey).send();
    checkbox.removeEventListener("keydown", preventDefault);
  });
  assert_equals(
    document.querySelector("input:active"),
    null,
    "The checkbox shouldn't be activated"
  );
}, "Space key shouldn't active the checkbox when its default is prevented");

promise_test(async t => {
  const radio = document.querySelector("input[type=radio]");
  radio.focus();
  radio.addEventListener("keydown", preventDefault);
  await (new test_driver.Actions()).keyDown(spaceKey).send();
  t.add_cleanup(async () => {
    await (new test_driver.Actions()).keyUp(spaceKey).send();
    radio.removeEventListener("keydown", preventDefault);
  });
  assert_equals(
    document.querySelector("input:active"),
    null,
    "The radio shouldn't be activated"
  );
}, "Space key shouldn't active the radio when its default is prevented");
</script>
</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/forms/the-input-element/checkable-active-space-key-prevented-default.html"
}
```
