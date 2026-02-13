# html/semantics/forms/the-button-element/button-activate-keyup-prevented.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-button-element/button-activate-keyup-prevented.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<title>Button activation submits on keyup, but not if keydown is defaultPrevented</title>
<link rel=help href="https://bugzilla.mozilla.org/show_bug.cgi?id=1481400">
<link rel=author href="mailto:emilio@crisal.io" title="Emilio Cobos Álvarez">
<link rel=author href="https://mozilla.org" title="Mozilla">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<button>The button</button>
<script>
let button = document.querySelector("button");
promise_test(async t => {
  button.focus();
  assert_equals(document.activeElement, button, "Button should be focused");
  let clickPromise = new Promise(resolve => {
    button.addEventListener("click", resolve, { once: true });
  });

  await test_driver.send_keys(button, " ");

  await clickPromise;

  assert_true(true, "Button should have activated");

  document.addEventListener("keydown", t.step_func(function(e) {
    e.preventDefault();
  }));

  button.addEventListener("click", t.unreached_func("button got incorrectly activated"));

  await test_driver.send_keys(button, " ");

  await new Promise(resolve => t.step_timeout(resolve, 0));
  assert_true(true, "Button should not have activated");
});
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
  "source_name": "html/semantics/forms/the-button-element/button-activate-keyup-prevented.html"
}
```
