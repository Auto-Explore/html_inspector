# html/semantics/forms/the-input-element/input-disabled-fieldset-dynamic.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-input-element/input-disabled-fieldset-dynamic.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<link rel="help" href="https://bugzilla.mozilla.org/show_bug.cgi?id=1861027">
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<script src=/resources/testdriver.js></script>
<script src=/resources/testdriver-actions.js></script>
<script src=/resources/testdriver-vendor.js></script>
<fieldset id="fieldset" disabled>
  <input id="target">
</fieldset>
<script>
const target = document.getElementById("target");
const fieldset = document.getElementById("fieldset");
promise_test(async function() {
  await new Promise(r => window.addEventListener("load", r, { once: true }));
  assert_true(target.matches(":disabled"), "Fieldset disables the input");
  assert_true(target.matches(":read-only"), "Disabled implies read-only");

  // Try to focus, it shouldn't be focusable.
  target.focus();

  assert_not_equals(document.activeElement, target, "Should not be focusable");

  fieldset.removeAttribute("disabled");

  assert_false(target.matches(":disabled"), "Should go back to writable");
  assert_false(target.matches(":read-only"), "No longer read-only");

  // Should be focusable now.
  target.focus();

  assert_equals(document.activeElement, target, "Should not be focusable");

  await test_driver.send_keys(target, "A");
  assert_equals(target.value, "A", "Typing should work");
});
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
  "source_name": "html/semantics/forms/the-input-element/input-disabled-fieldset-dynamic.html"
}
```
