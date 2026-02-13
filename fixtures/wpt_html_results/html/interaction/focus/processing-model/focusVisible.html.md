# html/interaction/focus/processing-model/focusVisible.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/interaction/focus/processing-model/focusVisible.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>

<title>focus(options) - focusVisible</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="/resources/testdriver-vendor.js"></script>

<style>
  div {
    height: 10px;
    border: 1px solid black;
  }
</style>

<button>ABC</button>
<input>
<div id="contenteditable" contenteditable></div>
<div id="tabindex" tabindex=0></div>
<div id="not_focusable"></div>

<div id="reset_focus" tabindex=0></div>

<script>
const reset_focus = document.getElementById("reset_focus");

async function check_focus_visible(element, options, { shouldBeVisible, shouldBeFocused }) {
  // Reset focus by clicking on a div, which should not show focus rings.
  await test_driver.click(reset_focus);

  assert_equals(document.activeElement, reset_focus, "Reset should be focused");
  assert_true(reset_focus.matches(":focus"), "Clicking focusable div should match :focus");
  assert_false(reset_focus.matches(":focus-visible"), "Clicking focusable div shouldn't match :focus-visible");

  element.focus(options);

  assert_equals(document.activeElement, shouldBeFocused ? element : reset_focus, "activeElement");
  assert_equals(element.matches(":focus"), shouldBeFocused, ":focus");
  assert_equals(element.matches(":focus-visible"), shouldBeVisible, ":focus-visible");
}

for (let selector of ["button", "input", "#contenteditable", "#tabindex", "#not_focusable"]) {
  promise_test(async function() {
    const takesKeyboardInput = selector == "#contenteditable" || selector == "input";
    const shouldBeFocused = selector != "#not_focusable";

    const element = document.querySelector(selector);

    await check_focus_visible(element, {}, {
      shouldBeVisible: takesKeyboardInput,
      shouldBeFocused,
    });

    await check_focus_visible(element, { focusVisible: true }, {
      shouldBeVisible: shouldBeFocused,
      shouldBeFocused,
    });
    await check_focus_visible(element, { focusVisible: false }, {
      shouldBeVisible: false,
      shouldBeFocused,
    });
  }, "FocusOptions.focusVisible: " + selector);
}
</script>
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
  "source_name": "html/interaction/focus/processing-model/focusVisible.html"
}
```
